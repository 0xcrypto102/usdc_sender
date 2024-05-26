use anchor_lang::prelude::*;

// pub const USDC_TOKEN_MINT_PUBKEY: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
// pub const USDC_TOKEN_MINT_PUBKEY: &str = "9nUq4Fka3feiCT96c9B4njbsVTSH8oALS65LL3fT937J";
pub const USDC_TOKEN_MINT_PUBKEY: &str = "8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE";

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub admin: Pubkey,
}

#[account]
#[derive(Default)]
pub struct UserPool {
    pub credit_amount: u64,
}
