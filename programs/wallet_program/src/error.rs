use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("This is the invalid  price feed")]
    InvalidPriceFeed,

    #[msg("Insufficient user balance")]
    InsufficientUserBalance
}
