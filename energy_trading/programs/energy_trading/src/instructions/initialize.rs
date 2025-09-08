use anchor_lang::prelude::*;
use crate::state::{Aggregator, Battery, Auction};

// Initialize aggregator account
#[derive(Accounts)]
pub struct InitializeAggregator<'info> {
    #[account(
        init,
        payer = payer,
        space = Aggregator::LEN,
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

pub fn initialize_aggregator(ctx: Context<InitializeAggregator>) -> Result<()> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let aggregator = &mut ctx.accounts.aggregator;
    aggregator.id = 1; // Simple ID for demo
    aggregator.authority = ctx.accounts.authority.key();
    aggregator.name = "Demo Aggregator".to_string();
    aggregator.reputation_score = 50;
    aggregator.successful_settlements = 0;
    aggregator.total_energy_traded = 0;
    aggregator.total_usdc_paid = 0;
    aggregator.created_at = current_timestamp;
    aggregator.last_activity = current_timestamp;

    msg!("Initialized aggregator account for: {:?}", ctx.accounts.authority.key());
    Ok(())
}

// Initialize battery account
#[derive(Accounts)]
pub struct InitializeBattery<'info> {
    #[account(
        init,
        payer = payer,
        space = Battery::LEN,
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

pub fn initialize_battery(ctx: Context<InitializeBattery>) -> Result<()> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let battery = &mut ctx.accounts.battery;
    battery.id = 1; // Simple ID for demo
    battery.owner = ctx.accounts.authority.key();
    battery.device_id = 1;
    battery.capacity_kwh = 15; // 15kWh for Australian home battery
    battery.total_energy_sold = 0;
    battery.total_usdc_earned = 0;
    battery.created_at = current_timestamp;
    battery.last_sale_at = None;

    msg!("Initialized battery account for: {:?}", ctx.accounts.authority.key());
    Ok(())
}

// Initialize auction account
#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct InitializeAuction<'info> {
    #[account(
        init,
        payer = payer,
        space = Auction::LEN,
        seeds = [b"auction", &auction_id.to_le_bytes()[..]],
        bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub aggregator: Account<'info, Aggregator>,
    #[account(mut)]
    pub battery: Account<'info, Battery>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_auction(
    ctx: Context<InitializeAuction>,
    auction_id: u64,
    energy_amount: u64,
    reserve_price: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;
    
    let auction = &mut ctx.accounts.auction;
    auction.id = auction_id;
    auction.aggregator_id = ctx.accounts.aggregator.id;
    auction.battery_id = ctx.accounts.battery.id;
    auction.energy_amount = energy_amount;
    auction.reserve_price = reserve_price;
    auction.final_price = None;
    auction.usdc_amount = None;
    auction.settled = false;
    auction.created_at = current_timestamp;
    auction.settled_at = None;
    auction.blockchain_tx_hash = None;

    msg!("Initialized auction {} for aggregator {} and battery {}", 
         auction_id, ctx.accounts.aggregator.id, ctx.accounts.battery.id);
    Ok(())
}

// Legacy initialize for backward compatibility
#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
