// Generated Anchor IDL types for energy_trading program
use anchor_lang::prelude::*;

declare_id!("4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb");

#[program]
pub mod energy_trading {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_aggregator(ctx: Context<InitializeAggregator>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_battery(ctx: Context<InitializeBattery>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
        auction_id: u64,
        energy_amount: u64,
        reserve_price: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn settle_auction(
        ctx: Context<SettleAuction>,
        auction_id: u64,
        energy_amount: u64,
        final_price: u64,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct InitializeAggregator<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 32 + 1 + 4 + 8 + 8 + 8 + 8,
        seeds = [b"aggregator", authority.key().as_ref()],
        bump
    )]
    pub aggregator: Account<'info, Aggregator>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeBattery<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 4 + 4 + 8 + 8 + 8 + 8,
        seeds = [b"battery", authority.key().as_ref()],
        bump
    )]
    pub battery: Account<'info, Battery>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(auction_id: u64, energy_amount: u64, reserve_price: u64)]
pub struct InitializeAuction<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub aggregator: Account<'info, Aggregator>,
    #[account(mut)]
    pub battery: Account<'info, Battery>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(auction_id: u64, energy_amount: u64, final_price: u64)]
pub struct SettleAuction<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub aggregator: Account<'info, Aggregator>,
    #[account(mut)]
    pub battery: Account<'info, Battery>,
    /// CHECK: Aggregator's USDC token account (payer)
    #[account(mut)]
    pub aggregator_usdc_account: UncheckedAccount<'info>,
    /// CHECK: BESS owner's USDC token account (receiver)
    #[account(mut)]
    pub battery_owner_usdc_account: UncheckedAccount<'info>,
    /// CHECK: USDC mint account
    pub usdc_mint: UncheckedAccount<'info>,
    pub aggregator_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

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
