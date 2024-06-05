use anchor_lang::{prelude::*};

use crate::{
    constants::*, state::Config, UserPool
};

use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token;

use std::mem::size_of;
use solana_program::{program::invoke_signed, system_instruction};
use pyth_solana_receiver_sdk::price_update::{
    get_feed_id_from_hex,
    PriceUpdateV2,
};


pub fn forward_usdc_to_admin(ctx: Context<ForwardUsdcToAdmin>, user_wallet_index: u32) -> Result<()> {
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

    ctx.accounts.user_pool.credit_amount += ctx.accounts.user_send_account.amount;
    ctx.accounts.user_pool.usdc_amount += ctx.accounts.user_send_account.amount;

    Ok(())
}

pub fn forward_usdt_to_admin(ctx: Context<ForwardUsdTtoAdmin>, user_wallet_index: u32) -> Result<()> {
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

    let usdt_price_update = &mut ctx.accounts.usdt_price_update;
    let usdc_price_update = &mut ctx.accounts.usdc_price_update;

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

    let usdc_amount = ctx.accounts.user_send_account.amount * u64::try_from(usdt_current_price.price).unwrap() / 10u64.pow(u32::try_from(-usdt_current_price.exponent).unwrap()) * 10u64.pow(u32::try_from(-usdc_current_price.exponent).unwrap()) / u64::try_from(usdc_current_price.price).unwrap() ;

    ctx.accounts.user_pool.credit_amount += usdc_amount;
    ctx.accounts.user_pool.usdt_amount += ctx.accounts.user_send_account.amount;


    Ok(())
}


pub fn forward_sol_to_admin(ctx: Context<ForwardSolToAdmin>, user_wallet_index: u32, amount: u64) -> Result<()> {
    let binding = user_wallet_index.to_le_bytes();
    let (_, bump) = Pubkey::find_program_address(&[USER_WALLET,  binding.as_ref()], &ctx.program_id);
    let vault_seeds = &[USER_WALLET,binding.as_ref(), &[bump]];
    let signer = &[&vault_seeds[..]];

    invoke_signed(
        &system_instruction::transfer(&ctx.accounts.user_wallet.key(), &ctx.accounts.master_wallet.key(), amount),
        &[
            ctx.accounts.user_wallet.to_account_info().clone(),
            ctx.accounts.master_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
        signer,
    )?;

    let sol_price_update = &mut ctx.accounts.sol_price_update;
    let usdc_price_update = &mut ctx.accounts.usdc_price_update;

    // 1-Fetch latest price of usdc
    let usdc_current_price = usdc_price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(USDC_PIRCE_FEED)?,
    )?;

    // 2-Fetch latest price of sol
    let sol_current_price = sol_price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(SOL_PIRCE_FEED)?,
    )?;
    // 3-Format display values rounded to nearest dollar
    let sol_amount = amount * u64::try_from(sol_current_price.price).unwrap() / 10u64.pow(u32::try_from(-sol_current_price.exponent).unwrap()) * 10u64.pow(u32::try_from(-usdc_current_price.exponent).unwrap()) / u64::try_from(usdc_current_price.price).unwrap() / 1000 ;// 1000 = sol lamports - usdc decimals

    ctx.accounts.user_pool.credit_amount += sol_amount;
    ctx.accounts.user_pool.sol_amount += amount;

    Ok(())

}



#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct ForwardUsdcToAdmin<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [TOKEN_VAULT, user_wallet_index.to_le_bytes().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = user_wallet,
    )]
    pub user_send_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub vault_receive_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + size_of::<UserPool>(),
        seeds = [USER_AUTHORITY, user_wallet_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_pool: Box<Account<'info, UserPool>>,
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
pub struct ForwardUsdTtoAdmin<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [TOKEN_VAULT, user_wallet_index.to_le_bytes().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = user_wallet,
    )]
    pub user_send_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = config,
    )]
    pub vault_receive_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mint.key().to_string() == USDT_TOKEN_MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + size_of::<UserPool>(),
        seeds = [USER_AUTHORITY, user_wallet_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub usdc_price_update: Account<'info, PriceUpdateV2>,
    #[account(mut)]
    pub usdt_price_update: Account<'info, PriceUpdateV2>,

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
        init_if_needed,
        payer = authority,
        space = 8 + size_of::<UserPool>(),
        seeds = [USER_AUTHORITY, user_wallet_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub sol_price_update: Account<'info, PriceUpdateV2>,
    #[account(mut)]
    pub usdc_price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
}

