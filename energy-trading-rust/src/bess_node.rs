use crate::etp_message::ETPMessage;
use crate::error::{Result, ETPError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tracing::info;

/// Energy status levels for BESS nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnergyStatus {
    Critical, // < 10% energy
    Low,     // 10-25% energy
    Normal,  // 25-75% energy
    High,    // > 75% energy
}

/// BESS (Battery Energy Storage System) Node
/// 
/// Represents a distributed battery energy storage system that can:
/// - Register with the energy trading network
/// - Respond to energy queries from aggregators
/// - Evaluate and respond to bids
/// - Maintain battery status and health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BESSNode {
    pub device_id: u64,
    pub device_name: String,
    pub total_energy_capacity: f64, // kWh
    pub current_energy_level: f64,  // kWh
    pub reserve_price: f64,         // cents/kWh - minimum price to accept
    pub max_discharge_rate: f64,    // kW
    pub battery_voltage: f64,       // V
    pub battery_health_status: u8,  // 0=excellent, 1=good, 2=fair, 3=poor
    pub percentage_for_sale: f64,   // % of energy available for trading
    pub is_online: bool,
    pub last_heartbeat: Option<std::time::SystemTime>,
}

impl BESSNode {
    /// Create a new BESS node
    pub fn new(
        device_id: u64,
        device_name: String,
        total_energy_capacity: f64,
        reserve_price: f64,
    ) -> Self {
        Self {
            device_id,
            device_name,
            total_energy_capacity,
            current_energy_level: total_energy_capacity * 0.8, // Start at 80% capacity
            reserve_price,
            max_discharge_rate: 5.0, // Default 5kW discharge rate
            battery_voltage: 12.6,   // Default 12.6V
            battery_health_status: 1, // Default good health
            percentage_for_sale: 50.0, // Default 50% available for sale
            is_online: true,
            last_heartbeat: Some(std::time::SystemTime::now()),
        }
    }

    /// Get the amount of energy available for sale
    pub fn get_available_energy(&self) -> f64 {
        self.current_energy_level * (self.percentage_for_sale / 100.0)
    }

    /// Check if the BESS can provide the requested energy amount
    pub fn can_provide_energy(&self, requested_energy: f64) -> bool {
        if requested_energy < 0.0 {
            return false;
        }
        if requested_energy == 0.0 {
            return true; // Zero energy requests are always acceptable (test/ping messages)
        }
        self.get_available_energy() >= requested_energy
    }

    /// Evaluate a bid and determine if it should be accepted
    pub fn evaluate_bid(&self, bid_price: f64, requested_energy: f64) -> BidEvaluation {
        if !self.can_provide_energy(requested_energy) {
            return BidEvaluation::Reject {
                reason: "Insufficient energy available".to_string(),
                code: 2,
            };
        }

        if !self.is_online {
            return BidEvaluation::Reject {
                reason: "BESS is offline".to_string(),
                code: 3,
            };
        }

        // Enhanced pricing based on energy status
        let energy_status = self.get_energy_status();
        let adjusted_reserve_price = match energy_status {
            EnergyStatus::Critical => self.reserve_price * 2.0, // Double price when critical
            EnergyStatus::Low => self.reserve_price * 1.5,      // 50% premium when low
            EnergyStatus::Normal => self.reserve_price,         // Normal price
            EnergyStatus::High => self.reserve_price * 0.9,     // 10% discount when high
        };

        if bid_price < adjusted_reserve_price {
            let reason = match energy_status {
                EnergyStatus::Critical => "Energy critical - only accepting premium bids".to_string(),
                EnergyStatus::Low => "Energy low - bid below adjusted reserve price".to_string(),
                _ => "Bid price below reserve price".to_string(),
            };
            return BidEvaluation::Reject {
                reason,
                code: 1,
            };
        }

        BidEvaluation::Accept {
            sale_price: bid_price,
            energy_amount: requested_energy,
        }
    }

    /// Update battery status
    pub fn update_battery_status(&mut self, energy_level: f64, voltage: f64, health: u8) {
        self.current_energy_level = energy_level;
        self.battery_voltage = voltage;
        self.battery_health_status = health;
        self.last_heartbeat = Some(std::time::SystemTime::now());
    }

    /// Set the percentage of energy available for sale
    pub fn set_percentage_for_sale(&mut self, percentage: f64) {
        self.percentage_for_sale = percentage.clamp(0.0, 100.0);
    }

    /// Generate a BESS status message
    pub fn generate_status_message(&self, message_id: u64) -> ETPMessage {
        ETPMessage::new_bess_status(
            message_id,
            self.device_id,
            self.current_energy_level,
            self.battery_health_status,
            self.battery_voltage,
            self.max_discharge_rate,
        )
    }

    /// Get the current energy status based on energy level
    pub fn get_energy_status(&self) -> EnergyStatus {
        let percentage = (self.current_energy_level / self.total_energy_capacity) * 100.0;
        match percentage {
            p if p < 10.0 => EnergyStatus::Critical,
            p if p < 25.0 => EnergyStatus::Low,
            p if p < 75.0 => EnergyStatus::Normal,
            _ => EnergyStatus::High,
        }
    }

    /// Sell energy and update the current energy level
    pub fn sell_energy(&mut self, energy_amount: f64) -> Result<()> {
        if !self.can_provide_energy(energy_amount) {
            return Err(ETPError::InsufficientEnergy);
        }
        self.current_energy_level -= energy_amount;
        info!("BESS {} sold {:.2} kWh, remaining: {:.2} kWh", 
              self.device_id, energy_amount, self.current_energy_level);
        Ok(())
    }

    /// Recharge energy over time (simulating solar charging)
    pub fn recharge_energy(&mut self, time_elapsed_seconds: f64) {
        let recharge_rate = 0.05; // 0.05 kWh per second (180 kWh/hour)
        let recharge_amount = recharge_rate * time_elapsed_seconds;
        let old_energy = self.current_energy_level;
        self.current_energy_level = (self.current_energy_level + recharge_amount)
            .min(self.total_energy_capacity);
        
        if self.current_energy_level > old_energy {
            info!("BESS {} recharged {:.2} kWh, new total: {:.2} kWh", 
                  self.device_id, self.current_energy_level - old_energy, self.current_energy_level);
        }
    }

    /// Check if BESS is depleted (no energy available for sale)
    pub fn is_depleted(&self) -> bool {
        self.get_available_energy() <= 0.1 // Less than 0.1 kWh available
    }

    /// Get energy percentage (0-100)
    pub fn get_energy_percentage(&self) -> f64 {
        (self.current_energy_level / self.total_energy_capacity) * 100.0
    }

    /// Generate a query response message
    pub fn generate_query_response(&self, message_id: u64, _aggregator_device_id: u64) -> ETPMessage {
        ETPMessage::new_query_response(
            message_id,
            self.device_id,
            self.current_energy_level,
            self.percentage_for_sale,
        )
    }
}

/// Result of bid evaluation
#[derive(Debug, Clone, PartialEq)]
pub enum BidEvaluation {
    Accept {
        sale_price: f64,
        energy_amount: f64,
    },
    Reject {
        reason: String,
        code: u8, // 1=price too low, 2=insufficient energy, 3=offline, etc.
    },
}

/// BESS Node Manager
/// 
/// Manages multiple BESS nodes and handles network communication
pub struct BESSNodeManager {
    pub nodes: Arc<RwLock<HashMap<u64, BESSNode>>>,
    pub listener: Option<TcpListener>,
    multicast_group: Ipv4Addr,
    multicast_port: u16,
}

impl Clone for BESSNodeManager {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            listener: None, // TcpListener can't be cloned, so we set to None
            multicast_group: self.multicast_group,
            multicast_port: self.multicast_port,
        }
    }
}

impl BESSNodeManager {
    /// Create a new BESS node manager
    pub fn new(multicast_group: Ipv4Addr, multicast_port: u16) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            listener: None,
            multicast_group,
            multicast_port,
        }
    }

    /// Add a BESS node to the manager
    pub async fn add_node(&self, node: BESSNode) -> Result<()> {
        let device_id = node.device_id;
        let mut nodes = self.nodes.write().await;
        nodes.insert(device_id, node);
        info!("Added BESS node: {}", device_id);
        Ok(())
    }

    /// Get a BESS node by ID
    pub async fn get_node(&self, device_id: u64) -> Option<BESSNode> {
        let nodes = self.nodes.read().await;
        nodes.get(&device_id).cloned()
    }

    /// Start the BESS node manager
    pub async fn start(&mut self, bind_addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        self.listener = Some(listener);
        info!("BESS Node Manager started on {}", bind_addr);
        Ok(())
    }

    /// Handle incoming TCP connections
    pub async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        let peer_addr = stream.peer_addr()?;
        info!("New connection from {}", peer_addr);

        // TODO: Implement message handling logic
        // This will be implemented in the next step

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bess_node_creation() {
        let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        assert_eq!(bess.device_id, 123);
        assert_eq!(bess.device_name, "BESS-001");
        assert_eq!(bess.total_energy_capacity, 100.0);
        assert_eq!(bess.current_energy_level, 80.0); // 80% of capacity
        assert_eq!(bess.reserve_price, 15.0);
        assert!(bess.is_online);
    }

    #[test]
    fn test_available_energy_calculation() {
        let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        bess.set_percentage_for_sale(50.0);
        assert_eq!(bess.get_available_energy(), 40.0); // 80 * 0.5
    }

    #[test]
    fn test_bid_evaluation_accept() {
        let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        let evaluation = bess.evaluate_bid(20.0, 10.0);
        
        match evaluation {
            BidEvaluation::Accept { sale_price, energy_amount } => {
                assert_eq!(sale_price, 20.0);
                assert_eq!(energy_amount, 10.0);
            }
            _ => panic!("Expected Accept evaluation"),
        }
    }

    #[test]
    fn test_bid_evaluation_reject_price_too_low() {
        let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        let evaluation = bess.evaluate_bid(10.0, 10.0);
        
        match evaluation {
            BidEvaluation::Reject { reason, code } => {
                assert_eq!(reason, "Bid price below reserve price");
                assert_eq!(code, 1);
            }
            _ => panic!("Expected Reject evaluation"),
        }
    }

    #[test]
    fn test_bid_evaluation_reject_insufficient_energy() {
        let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
        bess.set_percentage_for_sale(10.0); // Only 10% available
        let evaluation = bess.evaluate_bid(20.0, 50.0); // Request 50kWh but only 8kWh available
        
        match evaluation {
            BidEvaluation::Reject { reason, code } => {
                assert_eq!(reason, "Insufficient energy available");
                assert_eq!(code, 2);
            }
            _ => panic!("Expected Reject evaluation"),
        }
    }
}
