use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Auction, Aggregator, Battery};
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(auction_id: u64, energy_amount: u64, final_price: u64)]
pub struct SettleAuction<'info> {
    #[account(
        mut,
        seeds = [b"auction", &auction_id.to_le_bytes()[..]],
        bump
    )]
    pub auction: Account<'info, Auction>,
    
    #[account(
        mut,
        seeds = [b"aggregator", aggregator.authority.key().as_ref()],
        bump
    )]
    pub aggregator: Account<'info, Aggregator>,
    
    #[account(
        mut,
        seeds = [b"battery", battery.owner.key().as_ref()],
        bump
    )]
    pub battery: Account<'info, Battery>,
    
    /// Aggregator's USDC token account (payer)
    #[account(mut)]
    pub aggregator_usdc_account: Account<'info, TokenAccount>,
    
    /// BESS owner's USDC token account (receiver)
    #[account(mut)]
    pub battery_owner_usdc_account: Account<'info, TokenAccount>,
    
    /// USDC mint account
    pub usdc_mint: Account<'info, token::Mint>,
    
    /// Aggregator's authority
    pub aggregator_authority: Signer<'info>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SettleAuction>,
    auction_id: u64,
    energy_amount: u64,
    final_price: u64,
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let aggregator = &mut ctx.accounts.aggregator;
    let battery = &mut ctx.accounts.battery;
    
    // Validate auction is not already settled
    require!(!auction.settled, ErrorCode::AuctionAlreadySettled);
    
    // Validate auction belongs to this aggregator
    require_eq!(auction.aggregator_id, aggregator.id, ErrorCode::InvalidAggregator);
    
    // Validate auction belongs to this battery
    require_eq!(auction.battery_id, battery.id, ErrorCode::InvalidBattery);
    
    // Validate aggregator authority
    require_eq!(aggregator.authority, ctx.accounts.aggregator_authority.key(), ErrorCode::InvalidAggregator);
    
    // Validate energy amount is not zero
    require!(energy_amount > 0, ErrorCode::InvalidUsdcAmount);
    
    // Calculate USDC amount (price in cents * energy in kWh)
    let usdc_amount = (final_price * energy_amount) / 100; // Convert cents to dollars
    
    // Validate USDC amount is not zero
    require!(usdc_amount > 0, ErrorCode::InvalidUsdcAmount);
    
    // Check aggregator has sufficient USDC balance
    require!(
        ctx.accounts.aggregator_usdc_account.amount >= usdc_amount,
        ErrorCode::InsufficientUsdcBalance
    );
    
    // Transfer USDC from aggregator to BESS owner
    let transfer_instruction = Transfer {
        from: ctx.accounts.aggregator_usdc_account.to_account_info(),
        to: ctx.accounts.battery_owner_usdc_account.to_account_info(),
        authority: ctx.accounts.aggregator_authority.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );
    
    token::transfer(cpi_ctx, usdc_amount)?;
    
    // Update auction as settled
    auction.settled = true;
    auction.final_price = Some(final_price);
    auction.energy_amount = energy_amount;
    auction.usdc_amount = Some(usdc_amount);
    auction.settled_at = Some(Clock::get()?.unix_timestamp);
    
    // Update aggregator reputation (successful settlement)
    aggregator.successful_settlements += 1;
    aggregator.total_energy_traded += energy_amount;
    aggregator.reputation_score = aggregator.reputation_score
        .saturating_add(1)
        .min(100); // Cap at 100
    
    // Update battery owner stats
    battery.total_energy_sold += energy_amount;
    battery.total_usdc_earned += usdc_amount;
    battery.last_sale_at = Some(Clock::get()?.unix_timestamp);
    
    // Emit settlement event
    emit!(AuctionSettled {
        auction_id,
        aggregator_id: aggregator.id,
        battery_id: battery.id,
        energy_amount,
        final_price,
        usdc_amount,
        settled_at: auction.settled_at.unwrap_or(0),
    });
    
    Ok(())
}

#[event]
pub struct AuctionSettled {
    pub auction_id: u64,
    pub aggregator_id: u32,
    pub battery_id: u32,
    pub energy_amount: u64,
    pub final_price: u64,
    pub usdc_amount: u64,
    pub settled_at: i64,
}
