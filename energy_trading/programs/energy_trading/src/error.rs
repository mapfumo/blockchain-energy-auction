use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Auction has already been settled")]
    AuctionAlreadySettled,
    #[msg("Invalid aggregator for this auction")]
    InvalidAggregator,
    #[msg("Invalid battery for this auction")]
    InvalidBattery,
    #[msg("Insufficient USDC balance")]
    InsufficientUsdcBalance,
    #[msg("Invalid USDC amount")]
    InvalidUsdcAmount,
    #[msg("Auction not found")]
    AuctionNotFound,
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
}
