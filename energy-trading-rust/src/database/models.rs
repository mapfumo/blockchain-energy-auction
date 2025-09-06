use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Battery {
    pub id: i32,
    pub device_id: i32,
    pub owner_pubkey: String,
    pub energy_total: f64,
    pub percentage_for_sale: f64,
    pub reserve_price: f64,
    pub health_status: i32,
    pub voltage: f64,
    pub discharge_rate: f64,
    pub status: String,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Aggregator {
    pub id: i32,
    pub device_id: i32,
    pub owner_pubkey: String,
    pub max_bid_price: f64,
    pub energy_requirements: f64,
    pub reputation_score: i32,
    pub status: String,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Auction {
    pub id: i64,
    pub battery_id: i32,
    pub aggregator_id: Option<i32>,
    pub energy_amount: f64,
    pub reserve_price: f64,
    pub final_price: Option<f64>,
    pub status: String,
    pub blockchain_tx_hash: Option<String>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bid {
    pub id: i64,
    pub auction_id: i64,
    pub aggregator_id: i32,
    pub bid_price: f64,
    pub energy_amount: f64,
    pub status: String, // "pending", "accepted", "rejected"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_auctions: i64,
    pub total_bids: i64,
    pub avg_price_improvement_percent: f64,
    pub active_bess_nodes: i32,
    pub active_aggregators: i32,
}

// Insert/Update models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBattery {
    pub device_id: i32,
    pub owner_pubkey: String,
    pub energy_total: f64,
    pub percentage_for_sale: f64,
    pub reserve_price: f64,
    pub health_status: i32,
    pub voltage: f64,
    pub discharge_rate: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAggregator {
    pub device_id: i32,
    pub owner_pubkey: String,
    pub max_bid_price: f64,
    pub energy_requirements: f64,
    pub reputation_score: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAuction {
    pub battery_id: i32,
    pub energy_amount: f64,
    pub reserve_price: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBid {
    pub auction_id: i64,
    pub aggregator_id: i32,
    pub bid_price: f64,
    pub energy_amount: f64,
    pub status: String,
}
