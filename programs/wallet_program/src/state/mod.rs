use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub master_wallet: Pubkey,
    pub vault_usdc_account: Pubkey,
    pub vault_usdt_account: Pubkey,

    pub exchange_wallet: Pubkey,
    pub exchange_vault_usdc_account: Pubkey,
    pub exchange_vault_usdt_account: Pubkey,

    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct UserPool {
    pub sol_amount: u64,
    pub usdt_amount: u64,
    pub usdc_amount: u64,
}