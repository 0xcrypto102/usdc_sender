use anchor_lang::prelude::*;

use {
    crate::{
        constants::*, state::*, error::*,
    },
    anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
    },
};
use solana_program::{program::invoke_signed, system_instruction};

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user_pool = &mut ctx.accounts.user_pool;

    let (_, bump) = Pubkey::find_program_address(&[CONFIG], &ctx.program_id);
    let vault_seeds = &[CONFIG, &[bump]];
    let signer = &[&vault_seeds[..]];

    require!(user_pool.credit_amount >= amount, WalletError::InsufficientBalance);

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_send_account.to_account_info(),
                to: ctx.accounts.user_receive_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer,
        ),
        amount,
    )?;
    user_pool.credit_amount -= amount;

    Ok(())
}

pub fn withdraw_usdt(ctx: Context<WithdrawUsdt>, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.config;
    let user = &ctx.accounts.user;


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

pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    let destination = &accts.user;
    let source = &accts.master_wallet;
    let authority = &accts.config;
    let user = &accts.user;

    require!(user.key() == authority.authority, WalletError::NotOwnerAllowed);

    let seeds = &[MASTER_WALLET.as_ref(), &[ctx.bumps.master_wallet]];
    let signers = &[&seeds[..]];

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
        signers,
    )?;

    Ok(())
}
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub user_receive_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT, mint.key().as_ref()],
        bump
    )]
    pub vault_send_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [USER_AUTHORITY, authority.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawUsdt<'info> {
    #[account(
        seeds = [CONFIG],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(constraint = mint.key().to_string() == USDT_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = config
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = user,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct WithdrawSol<'info> {
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

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
