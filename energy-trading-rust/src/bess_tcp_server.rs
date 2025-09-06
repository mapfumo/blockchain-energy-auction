use crate::bess_node::{BESSNode, BidEvaluation};
use crate::etp_message::ETPMessage;
use crate::error::Result;
use crate::network::unicast_connection::UnicastConnection;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tracing::{info, warn, error};

/// BESS TCP Server
/// 
/// Handles TCP connections from aggregators and processes ETP messages.
/// Implements timing constraints and concurrent connection handling.
pub struct BESSTCPServer {
    bess_node: Arc<RwLock<BESSNode>>,
    listener: Option<TcpListener>,
    is_running: Arc<AtomicBool>,
    local_addr: Option<SocketAddr>,
}

impl BESSTCPServer {
    /// Create a new BESS TCP server
    pub async fn new(bess_node: BESSNode, bind_addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(bind_addr).await?;
        let local_addr = listener.local_addr()?;
        
        info!("BESS TCP Server created for device {} on {}", 
              bess_node.device_id, local_addr);
        
        Ok(Self {
            bess_node: Arc::new(RwLock::new(bess_node)),
            listener: Some(listener),
            is_running: Arc::new(AtomicBool::new(false)),
            local_addr: Some(local_addr),
        })
    }
    
    /// Start the TCP server
    pub async fn start(&mut self) -> Result<()> {
        let listener = self.listener.take()
            .ok_or_else(|| crate::error::ETPError::Network("Server not initialized".to_string()))?;
        
        self.is_running.store(true, Ordering::Relaxed);
        info!("BESS TCP Server started on {}", self.local_addr.unwrap());
        
        // Accept connections in a loop
        while self.is_running.load(Ordering::Relaxed) {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New connection from {}", addr);
                    
                    // Spawn a task to handle this connection
                    let bess_node = self.bess_node.clone();
                    let _is_running = self.is_running.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, addr, bess_node).await {
                            error!("Error handling connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    if self.is_running.load(Ordering::Relaxed) {
                        error!("Error accepting connection: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }
    
    /// Get local address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.local_addr.ok_or_else(|| 
            crate::error::ETPError::Network("Server not bound to address".to_string())
        )
    }
    
    /// Shutdown the server
    pub async fn shutdown(&self) -> Result<()> {
        self.is_running.store(false, Ordering::Relaxed);
        info!("BESS TCP Server shutdown requested");
        Ok(())
    }
    
    /// Handle a single TCP connection
    async fn handle_connection(
        stream: TcpStream, 
        addr: SocketAddr, 
        bess_node: Arc<RwLock<BESSNode>>
    ) -> Result<()> {
        let mut connection = UnicastConnection::new(stream);
        
        loop {
            match connection.receive_message().await {
                Ok(message) => {
                    // Process message with timing constraints
                    let start = std::time::Instant::now();
                    let max_delay = message.get_max_delay_ms();
                    match Self::process_message(message, &bess_node, &mut connection).await {
                        Ok(_) => {
                            let elapsed = start.elapsed().as_millis() as u64;
                            if elapsed > max_delay {
                                error!("Timing violation processing message from {}: {}ms > {}ms", 
                                       addr, elapsed, max_delay);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error processing message from {}: {}", addr, e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    if let crate::error::ETPError::Io(io_error) = &e {
                        if io_error.kind() == std::io::ErrorKind::UnexpectedEof {
                            info!("Connection closed by peer: {}", addr);
                        } else {
                            error!("Error receiving message from {}: {}", addr, io_error);
                        }
                    } else {
                        error!("Error processing message from {}: {}", addr, e);
                    }
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Process a received ETP message
    async fn process_message(
        message: ETPMessage,
        bess_node: &Arc<RwLock<BESSNode>>,
        connection: &mut UnicastConnection,
    ) -> Result<()> {
        let response = match message.message_type {
            0 => { // Register
                info!("Processing register message from device {}", message.device_id);
                None // No response needed for register
            }
            1 => { // Query
                info!("Processing query message from device {}", message.device_id);
                let bess = bess_node.read().await;
                Some(bess.generate_query_response(message.message_id, message.device_id))
            }
            2 => { // Query Response
                info!("Processing query response from device {}", message.device_id);
                None // BESS doesn't send query responses
            }
            3 => { // Bid
                info!("Processing bid message from device {}: ${:.2} for {:.2} kWh", 
                      message.device_id, message.bid_price, message.required_energy_amount);
                
                let bess = bess_node.read().await;
                let evaluation = bess.evaluate_bid(message.bid_price, message.required_energy_amount);
                
                match evaluation {
                    BidEvaluation::Accept { sale_price, energy_amount } => {
                        Some(ETPMessage::new_bid_accept(
                            message.message_id,
                            bess.device_id,
                            sale_price,
                            energy_amount,
                        ))
                    }
                    BidEvaluation::Reject { reason: _, code } => {
                        Some(ETPMessage::new_bid_reject(
                            message.message_id,
                            bess.device_id,
                            code,
                        ))
                    }
                }
            }
            4 => { // Bid Accept
                info!("Processing bid accept from device {}", message.device_id);
                None // BESS doesn't process bid accepts
            }
            5 => { // Bid Confirm
                info!("Processing bid confirm from device {}", message.device_id);
                None // BESS doesn't process bid confirms
            }
            6 => { // Bid Reject
                info!("Processing bid reject from device {}", message.device_id);
                None // BESS doesn't process bid rejects
            }
            7 => { // Terminate
                info!("Processing terminate message from device {}", message.device_id);
                None // No response needed for terminate
            }
            8 => { // Device Failure
                warn!("Processing device failure from device {}", message.device_id);
                None // No response needed for device failure
            }
            9 => { // BESS Status
                info!("Processing BESS status from device {}", message.device_id);
                None // BESS doesn't process status messages from others
            }
            _ => {
                warn!("Unknown message type: {}", message.message_type);
                None
            }
        };
        
        // Send response if needed
        if let Some(response_message) = response {
            connection.send_message(response_message).await?;
        }
        
        Ok(())
    }
}

impl Clone for BESSTCPServer {
    fn clone(&self) -> Self {
        Self {
            bess_node: self.bess_node.clone(),
            listener: None, // TcpListener can't be cloned
            is_running: self.is_running.clone(),
            local_addr: self.local_addr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_bess_tcp_server_creation() {
        let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        let server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
        
        assert!(!server.is_running().await);
        assert!(server.local_addr().is_ok());
    }

    #[tokio::test]
    async fn test_bess_tcp_server_startup() {
        let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
        let server_addr = server.local_addr().unwrap();
        
        // Start the server in a task
        let server_handle = tokio::spawn(async move {
            server.start().await.unwrap();
        });
        
        // Give server time to start
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Test connection
        let _client = TcpStream::connect(server_addr).await.unwrap();
        
        server_handle.abort();
    }
}
