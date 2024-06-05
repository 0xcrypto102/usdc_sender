use anchor_lang::prelude::*;

use crate::{
    state::Config,
    constants::{CONFIG, MASTER_WALLET, TOKEN_VAULT}
};

use anchor_spl::token::{Mint, Token, TokenAccount};


pub fn batch_withdraw(ctx: Context<BatchWithdraw>, amounts: Vec<u64>, address: Vec<Pubkey>) -> Result<()> {

    let (_, bump) = Pubkey::find_program_address(&[MASTER_WALLET], &ctx.program_id);
    let vault_seeds = &[MASTER_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

    for amount in amounts {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.vault_send_account.to_account_info(),
                    to: ctx.accounts.master_wallet.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct BatchWithdraw<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [TOKEN_VAULT, mint.key().as_ref()],
        bump
    )]
    pub vault_send_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

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