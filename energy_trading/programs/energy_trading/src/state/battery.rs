use anchor_lang::prelude::*;

#[account]
pub struct Battery {
    pub id: u32,
    pub owner: Pubkey,
    pub device_id: u32,
    pub capacity_kwh: u32,
    pub total_energy_sold: u64,
    pub total_usdc_earned: u64,
    pub created_at: i64,
    pub last_sale_at: Option<i64>,
}

impl Battery {
    pub const LEN: usize = 8 + // discriminator
        4 + // id
        32 + // owner
        4 + // device_id
        4 + // capacity_kwh
        8 + // total_energy_sold
        8 + // total_usdc_earned
        8 + // created_at
        1 + 8 + // last_sale_at (Option<i64>)
        32; // padding
}
