use {
    crate::{
        constants::*, state::*, error::*
    },
    anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
    },
    pyth_solana_receiver_sdk::price_update::{
        get_feed_id_from_hex,
        PriceUpdateV2,
    },
    std::mem::size_of
};
use solana_program::{program::invoke, system_instruction};
use std::str::FromStr;
use pyth_sdk_solana::load_price_feed_from_account_info;

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct DepositSol<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + size_of::<UserPool>(),
        seeds = [USER_AUTHORITY, user.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,
 
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_sol(ctx: Context<DepositSol>, user_wallet_index: u32,amount: u64) -> Result<()> {
    let destination = &ctx.accounts.user_wallet;
    let source = &ctx.accounts.user;
    let authority = &ctx.accounts.user;
   
    // Transfer sol from taker to initializer
    invoke(
        &system_instruction::transfer(
            &source.key(),
            &destination.key(),
            amount
        ),
        &[
            source.to_account_info().clone(),
            destination.clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    Ok(())
}
