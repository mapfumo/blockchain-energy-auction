use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
};
use std::str::FromStr;

// Program ID from our deployed contract
const PROGRAM_ID: &str = "4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb";
// USDC mint address (using a test USDC mint for local development)
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub struct BlockchainClient {
    rpc_client: RpcClient,
    payer: Keypair,
    program_id: Pubkey,
    usdc_mint: Pubkey,
}

impl BlockchainClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Connect to local Solana validator
        let payer = Keypair::new(); // Generate a new keypair for the gateway
        
        // Create RPC client
        let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
        
        // Get the program ID and USDC mint
        let program_id = Pubkey::from_str(PROGRAM_ID)?;
        let usdc_mint = Pubkey::from_str(USDC_MINT)?;
        
        Ok(Self {
            rpc_client,
            payer,
            program_id,
            usdc_mint,
        })
    }
    
    /// Initialize an auction on the blockchain
    pub async fn initialize_auction(
        &self,
        auction_id: u64,
        energy_amount: u64,
        reserve_price: u64,
        aggregator_keypair: &Keypair,
        battery_keypair: &Keypair,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        // For now, simulate a successful transaction
        // In a real implementation, this would create the proper transaction
        // and call the smart contract's initialize_auction instruction
        
        // Generate a realistic transaction signature
        let mut signature_bytes = [0u8; 64];
        signature_bytes[0..8].copy_from_slice(&auction_id.to_le_bytes());
        signature_bytes[8..16].copy_from_slice(&energy_amount.to_le_bytes());
        signature_bytes[16..24].copy_from_slice(&reserve_price.to_le_bytes());
        
        Ok(Signature::new_unique())
    }
    
    /// Settle an auction with USDC payment
    pub async fn settle_auction(
        &self,
        auction_id: u64,
        energy_amount: u64,
        final_price: u64,
        aggregator_keypair: &Keypair,
        battery_keypair: &Keypair,
        auction_pubkey: Pubkey,
    ) -> Result<Signature, Box<dyn std::error::Error>> {
        // For now, simulate a successful settlement transaction
        // In a real implementation, this would:
        // 1. Create/derive auction account address
        // 2. Create/derive aggregator and battery accounts
        // 3. Create USDC token accounts
        // 4. Call settle_auction instruction
        // 5. Transfer USDC from aggregator to battery
        
        // Generate a realistic transaction signature
        let mut signature_bytes = [0u8; 64];
        signature_bytes[0..8].copy_from_slice(&auction_id.to_le_bytes());
        signature_bytes[8..16].copy_from_slice(&energy_amount.to_le_bytes());
        signature_bytes[16..24].copy_from_slice(&final_price.to_le_bytes());
        signature_bytes[24..32].copy_from_slice(&auction_pubkey.to_bytes()[0..8]);
        
        Ok(Signature::new_unique())
    }
    
    /// Get auction data from blockchain
    pub async fn get_auction_data(
        &self,
        auction_pubkey: Pubkey,
    ) -> Result<AuctionData, Box<dyn std::error::Error>> {
        // For now, return simulated data
        // In a real implementation, this would fetch the account data from blockchain
        Ok(AuctionData {
            id: 42,
            energy_amount: 15000, // 15 kWh in Wh
            reserve_price: 645,   // 6.45 cents/kWh
            final_price: Some(645),
            settled: true,
            blockchain_tx_hash: Some("3Kx7...9mP2".to_string()),
        })
    }
    
    /// Get aggregator data from blockchain
    pub async fn get_aggregator_data(
        &self,
        aggregator_pubkey: Pubkey,
    ) -> Result<AggregatorData, Box<dyn std::error::Error>> {
        // For now, return simulated data
        Ok(AggregatorData {
            id: 2,
            reputation_score: 85,
            successful_settlements: 42,
            total_energy_traded: 356800, // 356.8 kWh in Wh
            total_usdc_paid: 12750,     // $127.50 in cents
        })
    }
    
    /// Get battery data from blockchain
    pub async fn get_battery_data(
        &self,
        battery_pubkey: Pubkey,
    ) -> Result<BatteryData, Box<dyn std::error::Error>> {
        // For now, return simulated data
        Ok(BatteryData {
            id: 1,
            capacity_kwh: 20,
            total_energy_sold: 15200, // 15.2 kWh in Wh
            total_usdc_earned: 9804,  // $98.04 in cents
        })
    }
    
    /// Get Solana Explorer URL for a transaction
    pub fn get_transaction_url(&self, signature: &Signature) -> String {
        format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature)
    }
    
    /// Get local Solana Explorer URL for a transaction
    pub fn get_local_transaction_url(&self, signature: &Signature) -> String {
        format!("http://localhost:3001/tx/{}", signature)
    }
}

// Data structures matching the IDL
#[derive(Debug, Clone)]
pub struct AuctionData {
    pub id: u64,
    pub energy_amount: u64,
    pub reserve_price: u64,
    pub final_price: Option<u64>,
    pub settled: bool,
    pub blockchain_tx_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AggregatorData {
    pub id: u32,
    pub reputation_score: u8,
    pub successful_settlements: u32,
    pub total_energy_traded: u64,
    pub total_usdc_paid: u64,
}

#[derive(Debug, Clone)]
pub struct BatteryData {
    pub id: u32,
    pub capacity_kwh: u32,
    pub total_energy_sold: u64,
    pub total_usdc_earned: u64,
}
