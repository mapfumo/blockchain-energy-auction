pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb");

#[program]
pub mod energy_trading {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn initialize_aggregator(ctx: Context<InitializeAggregator>) -> Result<()> {
        initialize::initialize_aggregator(ctx)
    }

    pub fn initialize_battery(ctx: Context<InitializeBattery>) -> Result<()> {
        initialize::initialize_battery(ctx)
    }

    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
        auction_id: u64,
        energy_amount: u64,
        reserve_price: u64,
    ) -> Result<()> {
        initialize::initialize_auction(ctx, auction_id, energy_amount, reserve_price)
    }

    pub fn settle_auction(
        ctx: Context<SettleAuction>,
        auction_id: u64,
        energy_amount: u64,
        final_price: u64,
    ) -> Result<()> {
        settle_auction::handler(ctx, auction_id, energy_amount, final_price)
    }
}
