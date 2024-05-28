use anchor_lang::prelude::*;

use crate::{
    state::Config,
    constants::{CONFIG, MASTER_WALLET, TOKEN_VAULT, USER_WALLET}
};

use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;

use std::mem::size_of;
pub fn withdraw(ctx: Context<Withdraw>,user_wallet_index: u8, amount: u64) -> Result<()> {
    let (_, bump) = Pubkey::find_program_address(&[MASTER_WALLET], &ctx.program_id);
    let vault_seeds = &[MASTER_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_send_account.to_account_info(),
                to: ctx.accounts.user_receive_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer,
        ),
        amount,
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(user_wallet_index: u8)]
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

    pub mint: Account<'info, Mint>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut, 
        seeds = [MASTER_WALLET], 
        bump
    )]
    pub master_wallet: AccountInfo<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}