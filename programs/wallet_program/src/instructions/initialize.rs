use anchor_lang::prelude::*;

use crate::{
    state::Config,
    constants::{CONFIG, MASTER_WALLET, TOKEN_VAULT}
};

use anchor_spl::token::{Mint, Token, TokenAccount};

use std::mem::size_of;

pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
    let accts = ctx.accounts;

    accts.config.authority = accts.authority.key();
    accts.config.master_wallet = accts.master_wallet.key();
    accts.config.vault_usdt_account = accts.vault_usdt_account.key();
    accts.config.bump = bump;

    Ok(())
}
  
pub fn initialize_usdc(ctx: Context<InitializeUsdc>) -> Result<()> {
    let accts = ctx.accounts;
    accts.config.vault_usdc_account = accts.vault_usdc_account.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init, 
        payer = authority, 
        seeds = [CONFIG], 
        bump,
        space = 8+ size_of::<Config>()
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [TOKEN_VAULT, usdt_mint.key().as_ref()],
        bump,
        token::mint = usdt_mint,
        token::authority = config,
    )]
    pub vault_usdt_account: Box<Account<'info, TokenAccount>>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut, 
        seeds = [MASTER_WALLET], 
        bump
    )]
    pub master_wallet: AccountInfo<'info>,
  
    #[account(mut)]
    pub usdt_mint: Account<'info, Mint>,
  
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUsdc<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut, 
        seeds = [CONFIG], 
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [TOKEN_VAULT, usdc_mint.key().as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = config,
    )]
    pub vault_usdc_account: Box<Account<'info, TokenAccount>>,
   
    #[account(mut)]
    pub usdc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

