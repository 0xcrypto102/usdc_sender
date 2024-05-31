use {
    crate::{
        constants::*, state::*
    },
    anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
    },
};
use solana_program::{program::invoke, system_instruction};
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(user_wallet_index: u8)]
pub struct DepositSol<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [USER_AUTHORITY, user.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    #[account(address = Pubkey::from_str(USDC_PIRCE_FEED).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub usdc_price_feed: AccountInfo<'info>,

    #[account(address = Pubkey::from_str(SOL_PIRCE_FEED).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub sol_price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
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

    // 1-Fetch latest price of usdc
    let price_usdc_account_info = &ctx.accounts.usdc_price_feed;
    let usdc_price_feed = load_price_feed_from_account_info( &price_usdc_account_info ).unwrap();
    let current_timestamp = Clock::get()?.unix_timestamp;
    let usdc_current_price = usdc_price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

    // 2-Fetch latest price of sol
    let price_sol_account_info = &ctx.accounts.sol_price_feed;
    let sol_price_feed = load_price_feed_from_account_info( &price_sol_account_info ).unwrap();
    let current_timestamp = Clock::get()?.unix_timestamp;
    let sol_current_price = sol_price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();


    // 3-Format display values rounded to nearest dollar
    let sol_amount = amount * u64::try_from(sol_current_price.price).unwrap() / 10u64.pow(u32::try_from(-sol_current_price.expo).unwrap()) * 10u64.pow(u32::try_from(-usdc_current_price.expo).unwrap()) / u64::try_from(usdc_current_price.price).unwrap() ;


    ctx.accounts.user_pool.credit_amount += sol_amount;
    ctx.accounts.user_pool.sol_amount += amount;

    Ok(())
}
