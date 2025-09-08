pub mod blockchain;

pub use blockchain::BlockchainClient;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BESSNode {
    pub node_id: String,
    pub energy_level: f64,
    pub capacity_kwh: f64,
    pub reserve_price: u32,
    pub is_online: bool,
    pub last_seen: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BESSStatus {
    pub node_id: String,
    pub energy_level: f64,
    pub capacity_kwh: f64,
    pub battery_health: f64,
    pub voltage: f64,
    pub temperature: f64,
    pub reserve_price: u32,
    pub is_online: bool,
    pub last_activity: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Aggregator {
    pub aggregator_id: String,
    pub strategy: String,
    pub total_energy_managed: f64,
    pub total_revenue: f64,
    pub is_online: bool,
    pub last_seen: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Auction {
    pub auction_id: String,
    pub total_energy: f64,
    pub reserve_price: u32,
    pub current_bid: Option<u32>,
    pub winner: Option<String>,
    pub status: String,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemMetrics {
    pub total_bess_nodes: u32,
    pub total_aggregators: u32,
    pub total_auctions: u32,
    pub total_energy_available: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockchainSettlement {
    pub auction_id: u64,
    pub winner: String,
    pub seller: String,
    pub energy_amount: f64,
    pub final_price: u32,
    pub total_value: f64,
    pub settlement_signature: String,
    pub blockchain_url: String,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub bess_nodes: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, BESSNode>>>,
    pub aggregators: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Aggregator>>>,
    pub auctions: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Auction>>>,
    pub event_tx: tokio::sync::broadcast::Sender<serde_json::Value>,
    pub blockchain_client: std::sync::Arc<tokio::sync::RwLock<Option<BlockchainClient>>>,
}

pub async fn trigger_blockchain_settlement(
    auction_id: u64,
    winner: String,
    seller: String,
    energy: f64,
    price: u32,
) -> bool {
    println!("🔗 trigger_blockchain_settlement called for auction #{}: {} -> {} ({} kWh at {}¢)", 
             auction_id, winner, seller, energy, price);
    
    // Create blockchain client
    let blockchain_client = match BlockchainClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Failed to create blockchain client: {}", e);
            return false;
        }
    };
    
    // Generate keypairs for aggregator and battery
    let aggregator_keypair = solana_sdk::signature::Keypair::new();
    let battery_keypair = solana_sdk::signature::Keypair::new();
    
    // Create auction PDA
    let auction_pubkey = solana_sdk::pubkey::Pubkey::find_program_address(
        &[b"auction", &auction_id.to_le_bytes()],
        blockchain_client.program_id(),
    ).0;
    
    // Convert energy to u64 (kWh * 1000 for precision)
    let energy_amount = (energy * 1000.0) as u64;
    
    // Settle the auction on blockchain
    match blockchain_client.settle_auction(
        auction_id,
        energy_amount,
        price as u64,
        &aggregator_keypair,
        &battery_keypair,
        auction_pubkey,
    ).await {
        Ok(signature) => {
            println!("✅ Real blockchain settlement successful: {}", signature);
            true
        }
        Err(e) => {
            eprintln!("❌ Blockchain settlement failed: {}", e);
            false
        }
    }
}
