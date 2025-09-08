use anchor_lang::prelude::*;

#[account]
pub struct Auction {
    pub id: u64,
    pub aggregator_id: u32,
    pub battery_id: u32,
    pub energy_amount: u64,
    pub reserve_price: u64,
    pub final_price: Option<u64>,
    pub usdc_amount: Option<u64>,
    pub settled: bool,
    pub created_at: i64,
    pub settled_at: Option<i64>,
    pub blockchain_tx_hash: Option<String>,
}

impl Auction {
    pub const LEN: usize = 8 + // discriminator
        8 + // id
        4 + // aggregator_id
        4 + // battery_id
        8 + // energy_amount
        8 + // reserve_price
        1 + 8 + // final_price (Option<u64>)
        1 + 8 + // usdc_amount (Option<u64>)
        1 + // settled
        8 + // created_at
        1 + 8 + // settled_at (Option<i64>)
        1 + 4 + 64 + // blockchain_tx_hash (Option<String> with max 64 chars)
        32; // padding
}
