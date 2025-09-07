use anchor_lang::prelude::*;

#[account]
pub struct Aggregator {
    pub id: u32,
    pub authority: Pubkey,
    pub name: String,
    pub reputation_score: u8,
    pub successful_settlements: u32,
    pub total_energy_traded: u64,
    pub total_usdc_paid: u64,
    pub created_at: i64,
    pub last_activity: i64,
}

impl Aggregator {
    pub const LEN: usize = 8 + // discriminator
        4 + // id
        32 + // authority
        4 + 32 + // name (String, max 32 chars)
        1 + // reputation_score
        4 + // successful_settlements
        8 + // total_energy_traded
        8 + // total_usdc_paid
        8 + // created_at
        8 + // last_activity
        32; // padding
}
