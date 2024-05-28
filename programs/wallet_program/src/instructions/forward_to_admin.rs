use anchor_lang::prelude::*;

use crate::{
    state::Config,
    constants::{CONFIG, MASTER_WALLET, TOKEN_VAULT, USER_WALLET}
};

use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;

use std::mem::size_of;

pub fn forward_to_admin(ctx: Context<ForwardToAdmin>, user_wallet_index: u32) -> Result<()> {
    let (_, bump) = Pubkey::find_program_address(&[USER_WALLET,  user_wallet_index.to_le_bytes().as_ref()], &ctx.program_id);
    let vault_seeds = &[USER_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_send_account.to_account_info(),
                to: ctx.accounts.vault_receive_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer,
        ),
        ctx.accounts.user_send_account.amount,
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(user_wallet_index: u8)]
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
    pub mint: Account<'info, Mint>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
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

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

