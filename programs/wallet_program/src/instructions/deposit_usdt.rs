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
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(user_wallet_index: u8)]
pub struct DepositUsdt<'info> {
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

    #[account(constraint = mint.key().to_string() == USDT_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user_wallet,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(address = Pubkey::from_str(USDC_PIRCE_FEED).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub usdc_price_feed: AccountInfo<'info>,

    #[account(address = Pubkey::from_str(USDT_PIRCE_FEED).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub usdt_price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DepositUsdt>, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.user;

    // Transfer tokens from taker to initializer
    let cpi_accounts = SplTransfer {
        from: source.to_account_info().clone(),
        to: destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();

    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    // 1-Fetch latest price of usdc
    let price_usdc_account_info = &ctx.accounts.usdc_price_feed;
    let usdc_price_feed = load_price_feed_from_account_info( &price_usdc_account_info ).unwrap();
    let current_timestamp = Clock::get()?.unix_timestamp;
    let usdc_current_price = usdc_price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

    // 2-Fetch latest price of usdt
    let price_usdt_account_info = &ctx.accounts.usdt_price_feed;
    let usdt_price_feed = load_price_feed_from_account_info( &price_usdt_account_info ).unwrap();
    let current_timestamp = Clock::get()?.unix_timestamp;
    let usdt_current_price = usdt_price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

    let usdc_amount = amount * u64::try_from(usdt_current_price.price).unwrap() / 10u64.pow(u32::try_from(-usdt_current_price.expo).unwrap()) * 10u64.pow(u32::try_from(-usdc_current_price.expo).unwrap()) / u64::try_from(usdc_current_price.price).unwrap() ;


    ctx.accounts.user_pool.credit_amount += usdc_amount;
    ctx.accounts.user_pool.usdt_amount += amount;

    Ok(())
}
