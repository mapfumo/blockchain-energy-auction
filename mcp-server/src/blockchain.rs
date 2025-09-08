use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
};
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

// Program ID from your deployed contract
const PROGRAM_ID: &str = "4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb";

pub struct BlockchainClient {
    rpc_client: RpcClient,
    program_id: Pubkey,
}

impl BlockchainClient {
    pub fn new() -> Result<Self> {
        // Connect to local Solana validator
        let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
        let program_id = Pubkey::from_str(PROGRAM_ID)?;
        
        Ok(Self {
            rpc_client,
            program_id,
        })
    }

    /// Verify a settlement transaction on the blockchain
    pub async fn verify_settlement_transaction(&self, signature: &str) -> Result<bool> {
        let sig = signature.parse::<Signature>()?;
        
        // Check if transaction exists and is confirmed
        match self.rpc_client.get_transaction(&sig, UiTransactionEncoding::Json) {
            Ok(transaction) => {
                // Transaction exists, check if it's confirmed
                Ok(transaction.transaction.meta.is_some())
            }
            Err(_) => {
                // Transaction not found
                Ok(false)
            }
        }
    }

    /// Get transaction details
    pub async fn get_transaction_details(&self, signature: &str) -> Result<Option<serde_json::Value>> {
        let sig = signature.parse::<Signature>()?;
        
        match self.rpc_client.get_transaction(&sig, UiTransactionEncoding::Json) {
            Ok(transaction) => {
                Ok(Some(serde_json::to_value(transaction)?))
            }
            Err(_) => {
                Ok(None)
            }
        }
    }

    /// Get recent settlement transactions
    pub async fn get_recent_settlements(&self, limit: usize) -> Result<Vec<String>> {
        // In a real implementation, this would query for settlement transactions
        // For now, return simulated transaction signatures
        let mut signatures = Vec::new();
        for i in 0..limit {
            signatures.push(format!("settlement_tx_{:08x}", i * 12345));
        }
        Ok(signatures)
    }

    /// Get Solana Explorer URL for a transaction
    pub fn get_explorer_url(&self, signature: &str) -> String {
        format!("https://explorer.solana.com/tx/{}?cluster=localnet", signature)
    }
}
