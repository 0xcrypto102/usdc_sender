use anchor_lang::{prelude::*};

use crate::{
    constants::*, state::Config, error::*,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
};

use std::mem::size_of;
use solana_program::{program::invoke_signed, system_instruction};

pub fn trasnfer_usdt_to_exchange(ctx: Context<TransferUsdtToExchange>, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_account;
    let source = &ctx.accounts.from_account;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.config;
    let user = &ctx.accounts.authority;

    require!(*user.key == authority.authority, WalletError::NotOwnerAllowed);

    // Transfer tokens from taker to initializer
    let cpi_accounts = SplTransfer {
        from: source.to_account_info().clone(),
        to: destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
    };

    let cpi_program = token_program.to_account_info();
    let seeds = &[CONFIG, &[ctx.bumps.config]];
    let signers = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signers),
        amount,
    )?;

    Ok(())
}

pub fn trasnfer_usdc_to_exchange(ctx: Context<TransferUsdcToExchange>, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_account;
    let source = &ctx.accounts.from_account;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.config;
    let user = &ctx.accounts.authority;

    require!(*user.key == authority.authority, WalletError::NotOwnerAllowed);

    // Transfer tokens from taker to initializer
    let cpi_accounts = SplTransfer {
        from: source.to_account_info().clone(),
        to: destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
    };

    let cpi_program = token_program.to_account_info();
    let seeds = &[CONFIG, &[ctx.bumps.config]];
    let signers = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signers),
        amount,
    )?;

    Ok(())
}

pub fn transfer_sol_to_exchange(ctx: Context<TransferSolToExchange>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    let destination = &accts.exchange_wallet;
    let source = &accts.master_wallet;
    let authority = &accts.config;
    let user = &accts.authority;

    require!(user.key() == authority.authority, WalletError::NotOwnerAllowed);

    let (_, bump) = Pubkey::find_program_address(&[MASTER_WALLET], &ctx.program_id);
    let vault_seeds = &[MASTER_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

    invoke_signed(
        &system_instruction::transfer(
            source.to_account_info().key,
            destination.to_account_info().key,
            amount,
        ),
        &[
            source.to_account_info(),
            destination.to_account_info(),
            accts.system_program.to_account_info(),
        ],
        signer,
    )?;

    Ok(())
}


#[derive(Accounts)]
pub struct TransferUsdcToExchange<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [EXCHANGE_CONFIG], 
        bump
    )]
    pub exchange_config: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub from_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = exchange_config,
    )]
    pub to_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferUsdtToExchange<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [EXCHANGE_CONFIG], 
        bump
    )]
    pub exchange_config: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub from_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = exchange_config,
    )]
    pub to_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mint.key().to_string() == USDT_TOKEN_MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSolToExchange<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut, 
        seeds = [MASTER_WALLET], 
        bump
    )]
    pub master_wallet: AccountInfo<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut, 
        seeds = [EXCHANGE_WALLET], 
        bump
    )]
    pub exchange_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

