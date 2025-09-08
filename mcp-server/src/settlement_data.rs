use serde::{Deserialize, Serialize};

/// Settlement status for an auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementStatus {
    pub auction_id: u64,
    pub settled: bool,
    pub settlement_signature: Option<String>,
    pub blockchain_url: Option<String>,
    pub timestamp: i64,
}

/// Detailed auction settlement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionSettlement {
    pub auction_id: u64,
    pub energy_amount_kwh: f64,
    pub final_price_cents: u64,
    pub total_value_usd: f64,
    pub settled: bool,
    pub settlement_signature: String,
    pub blockchain_url: String,
    pub timestamp: i64,
    pub winner: String,
    pub seller: String,
}

/// Aggregator performance and status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorStatus {
    pub aggregator_id: u32,
    pub reputation_score: u8,
    pub successful_settlements: u32,
    pub total_energy_traded_kwh: f64,
    pub total_usdc_paid: u64,
    pub last_settlement: i64,
}

/// Battery status and earnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryStatus {
    pub battery_id: u32,
    pub capacity_kwh: f64,
    pub total_energy_sold_kwh: f64,
    pub total_usdc_earned: u64,
    pub last_sale: i64,
    pub active: bool,
}

/// Settlement verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementVerification {
    pub signature: String,
    pub verified: bool,
    pub block_height: Option<u64>,
    pub confirmation_time: i64,
    pub explorer_url: String,
    pub settlement_data: Option<AuctionSettlement>,
}

/// Settlement monitoring summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementSummary {
    pub total_energy_traded: f64,
    pub total_value: f64,
    pub average_price: f64,
    pub settlement_count: usize,
    pub timestamp: i64,
}
