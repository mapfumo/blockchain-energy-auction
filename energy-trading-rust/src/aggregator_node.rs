use crate::etp_message::ETPMessage;
use crate::bess_node::BESSNode;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use rand::Rng;

/// Bidding strategy for aggregator nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiddingStrategy {
    Random,        // Random bid between reserve and max price
    Conservative,  // Bid close to reserve price
    Aggressive,    // Bid close to max price
    Intelligent,   // Use historical data and ML for optimal bidding
}

/// Historical bid data for learning and optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalBid {
    pub bess_device_id: u64,
    pub bid_price: f64,
    pub energy_amount: f64,
    pub was_accepted: bool,
    pub timestamp: std::time::SystemTime,
}

/// Bid response from BESS node
#[derive(Debug, Clone, PartialEq)]
pub enum BidResponse {
    Accept {
        sale_price: f64,
        energy_amount: f64,
    },
    Reject {
        reason: String,
        code: u8,
    },
}

/// Result of bid evaluation
#[derive(Debug, Clone, PartialEq)]
pub enum BidEvaluationResult {
    Accepted {
        final_price: f64,
        energy_amount: f64,
    },
    Rejected {
        reason: String,
        code: u8,
    },
}

/// Aggregator Node
/// 
/// Represents an energy aggregator that:
/// - Discovers and connects to BESS nodes
/// - Generates intelligent bids based on strategy
/// - Optimizes energy procurement across multiple BESS nodes
/// - Learns from historical bidding data
#[derive(Debug, Clone)]
pub struct AggregatorNode {
    pub device_id: u64,
    pub device_name: String,
    pub strategy: BiddingStrategy,
    pub is_online: bool,
    pub connected_bess_nodes: Arc<RwLock<HashMap<u64, BESSNode>>>,
    pub historical_bids: Arc<RwLock<Vec<HistoricalBid>>>,
    pub max_bid_price: f64,
    pub min_bid_price: f64,
}

/// Serializable version of AggregatorNode for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorNodeData {
    pub device_id: u64,
    pub device_name: String,
    pub strategy: BiddingStrategy,
    pub is_online: bool,
    pub max_bid_price: f64,
    pub min_bid_price: f64,
}

impl AggregatorNode {
    /// Create a new aggregator node
    pub fn new(device_id: u64, device_name: String, strategy: BiddingStrategy) -> Self {
        Self {
            device_id,
            device_name,
            strategy,
            is_online: true,
            connected_bess_nodes: Arc::new(RwLock::new(HashMap::new())),
            historical_bids: Arc::new(RwLock::new(Vec::new())),
            max_bid_price: 25.0, // Default max bid price
            min_bid_price: 10.0, // Default min bid price
        }
    }

    /// Generate a bid based on the current strategy
    pub async fn generate_bid(&self, reserve_price: f64, energy_amount: f64, max_price: f64) -> ETPMessage {
        let bid_price = match self.strategy {
            BiddingStrategy::Random => {
                let mut rng = rand::thread_rng();
                rng.gen_range(reserve_price..=max_price)
            }
            BiddingStrategy::Conservative => {
                reserve_price + 0.5 // Bid slightly above reserve
            }
            BiddingStrategy::Aggressive => {
                max_price - 0.5 // Bid slightly below max
            }
            BiddingStrategy::Intelligent => {
                self.predict_winning_price(energy_amount).await
            }
        };

        let mut bid = ETPMessage::new_bid(rand::thread_rng().gen_range(1000..=9999), bid_price, energy_amount);
        bid.device_id = self.device_id;
        bid
    }

    /// Predict winning price using historical data
    pub async fn predict_winning_price(&self, energy_amount: f64) -> f64 {
        let historical_bids = self.historical_bids.read().await;
        
        if historical_bids.is_empty() {
            // No historical data, use conservative approach
            return 15.0;
        }

        // Simple prediction based on recent successful bids
        let recent_successful: Vec<&HistoricalBid> = historical_bids
            .iter()
            .filter(|bid| bid.was_accepted && bid.energy_amount >= energy_amount * 0.8)
            .rev()
            .take(10)
            .collect();

        if recent_successful.is_empty() {
            return 15.0;
        }

        let avg_price: f64 = recent_successful.iter().map(|bid| bid.bid_price).sum::<f64>() / recent_successful.len() as f64;
        
        // Add small random variation to avoid identical bids
        let mut rng = rand::thread_rng();
        avg_price + rng.gen_range(-0.5..=0.5)
    }

    /// Add historical bid data
    pub async fn add_historical_bid(&self, bess_device_id: u64, bid_price: f64, energy_amount: f64, was_accepted: bool) {
        let mut historical_bids = self.historical_bids.write().await;
        historical_bids.push(HistoricalBid {
            bess_device_id,
            bid_price,
            energy_amount,
            was_accepted,
            timestamp: std::time::SystemTime::now(),
        });
    }

    /// Get success rate from historical data
    pub async fn get_success_rate(&self) -> f64 {
        let historical_bids = self.historical_bids.read().await;
        
        if historical_bids.is_empty() {
            return 0.0;
        }

        let successful_count = historical_bids.iter().filter(|bid| bid.was_accepted).count();
        successful_count as f64 / historical_bids.len() as f64
    }

    /// Get average bid price from historical data
    pub async fn get_average_bid_price(&self) -> f64 {
        let historical_bids = self.historical_bids.read().await;
        
        if historical_bids.is_empty() {
            return 0.0;
        }

        let total_price: f64 = historical_bids.iter().map(|bid| bid.bid_price).sum();
        total_price / historical_bids.len() as f64
    }

    /// Get bid history
    pub async fn get_bid_history(&self) -> Vec<HistoricalBid> {
        let historical_bids = self.historical_bids.read().await;
        historical_bids.clone()
    }

    /// Add connected BESS node
    pub async fn add_connected_bess(&self, device_id: u64, bess_node: BESSNode) {
        let mut connected_nodes = self.connected_bess_nodes.write().await;
        connected_nodes.insert(device_id, bess_node);
        info!("Added BESS node {} to aggregator {}", device_id, self.device_id);
    }

    /// Optimize bids across multiple BESS nodes
    pub async fn optimize_bids(&self, total_energy_required: f64, max_price: f64) -> Vec<ETPMessage> {
        let connected_nodes = self.connected_bess_nodes.read().await;
        
        if connected_nodes.is_empty() {
            return Vec::new();
        }

        let mut optimized_bids = Vec::new();
        let mut remaining_energy = total_energy_required;

        // Sort BESS nodes by price (lowest first)
        let mut sorted_bess: Vec<_> = connected_nodes.iter().collect();
        sorted_bess.sort_by(|a, b| a.1.reserve_price.partial_cmp(&b.1.reserve_price).unwrap());

        for (_device_id, bess_node) in sorted_bess {
            if remaining_energy <= 0.0 {
                break;
            }

            let available_energy = bess_node.get_available_energy();
            let energy_to_bid = remaining_energy.min(available_energy);

            if energy_to_bid > 0.0 {
                let bid = self.generate_bid(bess_node.reserve_price, energy_to_bid, max_price).await;
                optimized_bids.push(bid);
                remaining_energy -= energy_to_bid;
            }
        }

        optimized_bids
    }

    /// Evaluate bid response from BESS node
    pub async fn evaluate_bid_response(&self, bid: ETPMessage, response: BidResponse) -> BidEvaluationResult {
        match response {
            BidResponse::Accept { sale_price, energy_amount } => {
                // Record successful bid
                self.add_historical_bid(bid.device_id, bid.bid_price, energy_amount, true).await;
                
                BidEvaluationResult::Accepted {
                    final_price: sale_price,
                    energy_amount,
                }
            }
            BidResponse::Reject { reason, code } => {
                // Record failed bid
                self.add_historical_bid(bid.device_id, bid.bid_price, bid.required_energy_amount, false).await;
                
                BidEvaluationResult::Rejected { reason, code }
            }
        }
    }

    /// Set bidding strategy
    pub async fn set_strategy(&mut self, strategy: BiddingStrategy) {
        let strategy_name = format!("{:?}", strategy);
        self.strategy = strategy;
        info!("Aggregator {} switched to {} strategy", self.device_id, strategy_name);
    }

    /// Query BESS nodes for energy availability
    pub async fn query_bess_nodes(&self, energy_required: f64) -> Vec<ETPMessage> {
        let connected_nodes = self.connected_bess_nodes.read().await;
        let mut query_responses = Vec::new();

        for (_device_id, bess_node) in connected_nodes.iter() {
            if bess_node.can_provide_energy(energy_required) {
                let query_response = bess_node.generate_query_response(
                    rand::thread_rng().gen_range(1000..=9999),
                    self.device_id,
                );
                query_responses.push(query_response);
            }
        }

        query_responses
    }

    /// Place bids to BESS nodes
    pub async fn place_bids(&self, energy_required: f64, max_price: f64) -> Vec<ETPMessage> {
        self.optimize_bids(energy_required, max_price).await
    }

    /// Process bid response from BESS node
    pub async fn process_bid_response(&self, bid: ETPMessage) -> Result<()> {
        // Simulate processing bid response
        // In real implementation, this would handle network communication
        info!("Processing bid response for bid {}", bid.message_id);
        Ok(())
    }

    /// Convert to serializable data
    pub fn to_data(&self) -> AggregatorNodeData {
        AggregatorNodeData {
            device_id: self.device_id,
            device_name: self.device_name.clone(),
            strategy: self.strategy.clone(),
            is_online: self.is_online,
            max_bid_price: self.max_bid_price,
            min_bid_price: self.min_bid_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aggregator_creation() {
        let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
        assert_eq!(aggregator.device_id, 123);
        assert_eq!(aggregator.device_name, "AGG-001");
        assert_eq!(aggregator.strategy, BiddingStrategy::Intelligent);
    }

    #[tokio::test]
    async fn test_bid_generation() {
        let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Random);
        let bid = aggregator.generate_bid(15.0, 10.0, 20.0).await;
        
        assert_eq!(bid.message_type, 3); // Bid message type
        assert_eq!(bid.device_id, 123);
        assert!(bid.bid_price >= 15.0);
        assert!(bid.bid_price <= 20.0);
        assert_eq!(bid.required_energy_amount, 10.0);
    }

    #[tokio::test]
    async fn test_historical_bid_tracking() {
        let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
        
        aggregator.add_historical_bid(100, 15.5, 10.0, true).await;
        aggregator.add_historical_bid(101, 16.0, 8.0, false).await;
        
        let success_rate = aggregator.get_success_rate().await;
        assert_eq!(success_rate, 0.5);
        
        let avg_price = aggregator.get_average_bid_price().await;
        assert_eq!(avg_price, 15.75);
    }
}
