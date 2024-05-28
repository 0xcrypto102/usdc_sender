use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub master_wallet: Pubkey,
    pub vault_usdc_account: Pubkey,
    pub vault_usdt_account: Pubkey,
    pub bump: u8,
}
