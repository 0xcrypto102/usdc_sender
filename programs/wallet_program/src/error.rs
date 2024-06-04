use anchor_lang::prelude::*;

#[error_code]
pub enum WalletError {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Not owner allowed")]
    NotOwnerAllowed,

    #[msg("This is the invalid  price feed")]
    InvalidPriceFeed,

    #[msg("Insufficient user balance")]
    InsufficientUserBalance,

    #[msg("Insufficient balance")]
    InsufficientBalance
}
