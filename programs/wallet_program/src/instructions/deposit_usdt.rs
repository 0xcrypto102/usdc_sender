use {
    crate::{
        constants::*, error::*, state::*
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
    std::mem::size_of,
};
use std::str::FromStr;
use pyth_sdk_solana::load_price_feed_from_account_info;

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct DepositUsdt<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + size_of::<UserPool>(),
        seeds = [USER_AUTHORITY, user.key().as_ref()],
        bump,
    )]
    pub user_pool: Box<Account<'info, UserPool>>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    #[account(constraint = mint.key().to_string() == USDT_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user
    )]
    pub from_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [TOKEN_VAULT, user.key.as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = user_wallet,
    )]
    pub to_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub usdc_price_update:  Account<'info, PriceUpdateV2>,
    #[account(mut)]
    pub usdt_price_update:  Account<'info, PriceUpdateV2>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_usdt(ctx: Context<DepositUsdt>,user_wallet_index: u32, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.user;
    let usdt_price_update = &mut ctx.accounts.usdt_price_update;
    let usdc_price_update = &mut ctx.accounts.usdc_price_update;

    // Transfer tokens from taker to initializer
    let cpi_accounts = SplTransfer {
        from: source.to_account_info().clone(),
        to: destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();

    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    // 1-Fetch latest price of usdc
    let usdc_current_price = usdc_price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(USDC_PIRCE_FEED)?,
    )?;

    // 2-Fetch latest price of sol
    let usdt_current_price = usdt_price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(USDT_PIRCE_FEED)?,
    )?;

    let usdc_amount = amount * u64::try_from(usdt_current_price.price).unwrap() / 10u64.pow(u32::try_from(-usdt_current_price.exponent).unwrap()) * 10u64.pow(u32::try_from(-usdc_current_price.exponent).unwrap()) / u64::try_from(usdc_current_price.price).unwrap() ;


    ctx.accounts.user_pool.credit_amount += usdc_amount;
    ctx.accounts.user_pool.usdt_amount += amount;

    Ok(())
}
