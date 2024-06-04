use anchor_lang::{prelude::*};

use crate::{
    constants::*, state::Config, UserPool
};

use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;

use std::mem::size_of;
use solana_program::{program::invoke_signed, system_instruction};


pub fn forward_to_admin(ctx: Context<ForwardToAdmin>, user_wallet_index: u32) -> Result<()> {
    let binding = user_wallet_index.to_le_bytes();
    let (_, bump) = Pubkey::find_program_address(&[USER_WALLET,  binding.as_ref()], &ctx.program_id);
    let vault_seeds = &[USER_WALLET, binding.as_ref(), &[bump]];
    let signer = &[&vault_seeds[..]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_send_account.to_account_info(),
                to: ctx.accounts.vault_receive_account.to_account_info(),
                authority: ctx.accounts.user_wallet.to_account_info(),
            },
            signer,
        ),
        ctx.accounts.user_send_account.amount,
    )?;
    
    if ctx.accounts.mint.key() == Pubkey::try_from(USDC_TOKEN_MINT_PUBKEY).unwrap() {
        ctx.accounts.user_pool.usdc_amount = 0;
    }

    if ctx.accounts.mint.key() == Pubkey::try_from(USDT_TOKEN_MINT_PUBKEY).unwrap() {
        ctx.accounts.user_pool.usdt_amount = 0;
    }

    Ok(())
}

pub fn forward_sol_to_admin(ctx: Context<ForwardSolToAdmin>, user_wallet_index: u32) -> Result<()> {
    let binding = user_wallet_index.to_le_bytes();
    let (_, bump) = Pubkey::find_program_address(&[USER_WALLET,  binding.as_ref()], &ctx.program_id);
    let vault_seeds = &[USER_WALLET,binding.as_ref(), &[bump]];
    let signer = &[&vault_seeds[..]];

    let amount = ctx.accounts.user_pool.sol_amount;

    invoke_signed(
        &system_instruction::transfer(&ctx.accounts.user_wallet.key(), &ctx.accounts.master_wallet.key(), amount),
        &[
            ctx.accounts.user_wallet.to_account_info().clone(),
            ctx.accounts.master_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
        signer,
    )?;

    ctx.accounts.user_pool.sol_amount = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct ForwardToAdmin<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = user_wallet
    )]
    pub user_send_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub vault_receive_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_AUTHORITY, user.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct ForwardSolToAdmin<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut, 
        seeds = [MASTER_WALLET], 
        bump
    )]
    pub master_wallet: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_AUTHORITY, authority.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

