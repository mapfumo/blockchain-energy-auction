use crate::etp_message::ETPMessage;
use crate::bess_node::BESSNode;
use crate::error::Result;
use rand::Rng;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Multicast Discovery Service
/// 
/// Handles BESS node registration and discovery queries using multicast UDP.
/// Based on the research paper specifications for service discovery.
pub struct MulticastDiscovery {
    pub multicast_group: Ipv4Addr,
    pub multicast_port: u16,
    socket: UdpSocket,
    registered_bess_nodes: Arc<RwLock<HashMap<u64, BESSNode>>>,
    is_running: Arc<RwLock<bool>>,
}

impl MulticastDiscovery {
    /// Create a new multicast discovery service
    pub async fn new(multicast_group: Ipv4Addr, multicast_port: u16) -> Result<Self> {
        // Validate multicast address
        if !multicast_group.is_multicast() {
            return Err(crate::error::ETPError::Network(
                format!("Invalid multicast address: {}", multicast_group)
            ));
        }

        // Create UDP socket for multicast
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.join_multicast_v4(multicast_group, Ipv4Addr::new(0, 0, 0, 0))?;
        
        info!("Multicast discovery started on {}:{}", multicast_group, multicast_port);

        Ok(Self {
            multicast_group,
            multicast_port,
            socket,
            registered_bess_nodes: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(RwLock::new(true)),
        })
    }

    /// Register a BESS node
    pub async fn register_bess_node(&self, bess_node: BESSNode) -> Result<()> {
        let device_id = bess_node.device_id;
        let mut nodes = self.registered_bess_nodes.write().await;
        nodes.insert(device_id, bess_node);
        
        info!("Registered BESS node: {}", device_id);
        
        // Broadcast registration event
        self.broadcast_registration_event(device_id).await?;
        
        Ok(())
    }

    /// Handle discovery query from aggregators
    pub async fn handle_discovery_query(&self, query: ETPMessage) -> Result<Vec<ETPMessage>> {
        let mut responses = Vec::new();
        let nodes = self.registered_bess_nodes.read().await;
        
        for (_device_id, bess_node) in nodes.iter() {
            if bess_node.is_online && bess_node.get_available_energy() > 0.0 {
                let response = bess_node.generate_query_response(
                    rand::thread_rng().gen_range(1000..=9999),
                    query.device_id,
                );
                responses.push(response);
            }
        }
        
        info!("Discovery query from aggregator {} returned {} BESS nodes", 
              query.device_id, responses.len());
        
        Ok(responses)
    }

    /// Get all registered BESS nodes
    pub async fn get_registered_bess_nodes(&self) -> Vec<BESSNode> {
        let nodes = self.registered_bess_nodes.read().await;
        nodes.values().cloned().collect()
    }

    /// Check if discovery service is running
    pub async fn is_running(&self) -> bool {
        let running = self.is_running.read().await;
        *running
    }

    /// Start the discovery service
    pub async fn start(&self) -> Result<()> {
        let mut running = self.is_running.write().await;
        *running = true;
        
        info!("Multicast discovery service started");
        Ok(())
    }

    /// Stop the discovery service
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.is_running.write().await;
        *running = false;
        
        info!("Multicast discovery service stopped");
        Ok(())
    }

    /// Broadcast registration event
    async fn broadcast_registration_event(&self, device_id: u64) -> Result<()> {
        let register_message = ETPMessage::new_register(
            rand::thread_rng().gen_range(1000..=9999),
            device_id,
        );
        
        let serialized = register_message.serialize()?;
        let multicast_addr = SocketAddr::new(self.multicast_group.into(), self.multicast_port);
        
        self.socket.send_to(&serialized, multicast_addr).await?;
        
        Ok(())
    }

    /// Listen for multicast messages
    pub async fn listen(&self) -> Result<()> {
        let mut buffer = [0u8; 1024];
        
        loop {
            let (len, addr) = self.socket.recv_from(&mut buffer).await?;
            
            // Deserialize message
            match ETPMessage::deserialize(&buffer[..len]) {
                Ok(message) => {
                    self.handle_multicast_message(message, addr).await?;
                }
                Err(e) => {
                    warn!("Failed to deserialize multicast message from {}: {}", addr, e);
                }
            }
        }
    }

    /// Handle incoming multicast message
    async fn handle_multicast_message(&self, message: ETPMessage, addr: SocketAddr) -> Result<()> {
        match message.message_type {
            0 => { // Register message
                info!("Received registration from {} at {}", message.device_id, addr);
            }
            1 => { // Query message
                info!("Received discovery query from {} at {}", message.device_id, addr);
                let responses = self.handle_discovery_query(message).await?;
                
                // Send responses back to querying aggregator
                for response in responses {
                    let serialized = response.serialize()?;
                    self.socket.send_to(&serialized, addr).await?;
                }
            }
            _ => {
                warn!("Unknown multicast message type: {}", message.message_type);
            }
        }
        
        Ok(())
    }
}

impl Clone for MulticastDiscovery {
    fn clone(&self) -> Self {
        // For testing purposes, we'll create a simple clone
        // In production, this would need proper socket management
        let std_socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
        std_socket.set_nonblocking(true).unwrap();
        let socket = UdpSocket::from_std(std_socket).unwrap();
        
        Self {
            multicast_group: self.multicast_group,
            multicast_port: self.multicast_port,
            socket,
            registered_bess_nodes: self.registered_bess_nodes.clone(),
            is_running: self.is_running.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_multicast_discovery_creation() {
        let discovery = MulticastDiscovery::new(
            Ipv4Addr::new(224, 0, 0, 1),
            8888,
        ).await.unwrap();
        
        assert_eq!(discovery.multicast_group, Ipv4Addr::new(224, 0, 0, 1));
        assert_eq!(discovery.multicast_port, 8888);
    }

    #[tokio::test]
    async fn test_bess_node_registration() {
        let discovery = MulticastDiscovery::new(
            Ipv4Addr::new(224, 0, 0, 1),
            8888,
        ).await.unwrap();
        
        let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        discovery.register_bess_node(bess).await.unwrap();
        
        let nodes = discovery.get_registered_bess_nodes().await;
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].device_id, 123);
    }
}
