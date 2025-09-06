use crate::etp_message::ETPMessage;
use crate::error::Result;
use std::io::ErrorKind;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, warn, error};

/// Unicast TCP Connection
/// 
/// Handles reliable message delivery between aggregators and BESS nodes.
/// Implements message framing for TCP streams as per ETP specifications.
pub struct UnicastConnection {
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl UnicastConnection {
    /// Create a new unicast connection
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::new(),
        }
    }

    /// Send an ETP message over the connection
    pub async fn send_message(&mut self, message: ETPMessage) -> Result<()> {
        let serialized = message.serialize()?;
        
        // Add message length prefix (4 bytes)
        let length = serialized.len() as u32;
        let mut framed_message = Vec::new();
        framed_message.extend_from_slice(&length.to_le_bytes());
        framed_message.extend_from_slice(&serialized);
        
        // Send the framed message
        self.stream.write_all(&framed_message).await?;
        self.stream.flush().await?;
        
        info!("Sent ETP message type {} ({} bytes)", message.message_type, serialized.len());
        Ok(())
    }

    /// Receive an ETP message from the connection
    pub async fn receive_message(&mut self) -> Result<ETPMessage> {
        // Read message length (4 bytes)
        let mut length_bytes = [0u8; 4];
        self.stream.read_exact(&mut length_bytes).await?;
        let message_length = u32::from_le_bytes(length_bytes) as usize;
        
        // Read the actual message
        let mut message_bytes = vec![0u8; message_length];
        self.stream.read_exact(&mut message_bytes).await?;
        
        // Deserialize the message
        let message = ETPMessage::deserialize(&message_bytes)?;
        
        info!("Received ETP message type {} ({} bytes)", message.message_type, message_length);
        Ok(message)
    }

    /// Handle incoming messages (for server-side connections)
    pub async fn handle_messages(&mut self) -> Result<()> {
        loop {
            match self.receive_message().await {
                Ok(message) => {
                    self.process_message(message).await?;
                }
                Err(e) => {
                    // Check if it's an IO error
                    if let crate::error::ETPError::Io(io_error) = &e {
                        if io_error.kind() == ErrorKind::UnexpectedEof {
                            info!("Connection closed by peer");
                            break;
                        } else {
                            error!("Error receiving message: {}", io_error);
                            return Err(e);
                        }
                    } else {
                        error!("Error receiving message: {}", e);
                        return Err(e);
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Process received message
    async fn process_message(&mut self, message: ETPMessage) -> Result<()> {
        match message.message_type {
            0 => { // Register
                info!("Processing register message from device {}", message.device_id);
            }
            1 => { // Query
                info!("Processing query message from device {}", message.device_id);
            }
            2 => { // Query Response
                info!("Processing query response from device {}", message.device_id);
            }
            3 => { // Bid
                info!("Processing bid message from device {}: ${:.2} for {:.2} kWh", 
                      message.device_id, message.bid_price, message.required_energy_amount);
            }
            4 => { // Bid Accept
                info!("Processing bid accept from device {}", message.device_id);
            }
            5 => { // Bid Confirm
                info!("Processing bid confirm from device {}", message.device_id);
            }
            6 => { // Bid Reject
                info!("Processing bid reject from device {}", message.device_id);
            }
            7 => { // Terminate
                info!("Processing terminate message from device {}", message.device_id);
            }
            8 => { // Device Failure
                warn!("Processing device failure from device {}", message.device_id);
            }
            9 => { // BESS Status
                info!("Processing BESS status from device {}", message.device_id);
            }
            _ => {
                warn!("Unknown message type: {}", message.message_type);
            }
        }
        
        Ok(())
    }

    /// Check if connection is still alive
    pub async fn is_alive(&mut self) -> bool {
        // Try to read 1 byte with a very short timeout
        let mut buffer = [0u8; 1];
        match self.stream.try_read(&mut buffer) {
            Ok(0) => false, // Connection closed
            Ok(_) => true,  // Data available
            Err(_) => true, // Error reading (connection might still be alive)
        }
    }

    /// Get peer address
    pub fn peer_addr(&self) -> Result<std::net::SocketAddr> {
        self.stream.peer_addr().map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_unicast_connection_creation() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        
        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            UnicastConnection::new(stream)
        });
        
        let client_stream = TcpStream::connect(server_addr).await.unwrap();
        let client_connection = UnicastConnection::new(client_stream);
        
        assert!(client_connection.peer_addr().is_ok());
        server_handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_message_send_receive() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        
        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut connection = UnicastConnection::new(stream);
            
            let received_message = connection.receive_message().await.unwrap();
            assert_eq!(received_message.message_type, 3); // Bid message
            assert_eq!(received_message.bid_price, 18.0);
        });
        
        let client_stream = TcpStream::connect(server_addr).await.unwrap();
        let mut client_connection = UnicastConnection::new(client_stream);
        
        let test_message = ETPMessage::new_bid(123, 18.0, 10.0);
        client_connection.send_message(test_message).await.unwrap();
        
        server_handle.await.unwrap();
    }
}
