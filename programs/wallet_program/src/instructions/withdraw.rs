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

pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>,user_wallet_index: u32, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.config;
    let user = &ctx.accounts.user;
    let user_pool = &mut ctx.accounts.user_pool;

    require!(user_pool.usdc_amount > amount, WalletError::InsufficientBalance);
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

    user_pool.usdc_amount -= amount;

    Ok(())
}

pub fn withdraw_usdt(ctx: Context<WithdrawUsdt>,user_wallet_index: u32, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.config;
    let user = &ctx.accounts.user;
    let user_pool = &mut ctx.accounts.user_pool;

    require!(user_pool.usdt_amount > amount, WalletError::InsufficientBalance);
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

    user_pool.usdt_amount -= amount;

    Ok(())
}

pub fn withdraw_sol(ctx: Context<WithdrawSol>, user_wallet_index: u32,amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    let destination = &accts.receiver;
    let source = &accts.master_wallet;
    let authority = &accts.config;
    let user = &accts.user;
    let user_pool = &mut accts.user_pool;

    require!(user_pool.sol_amount > amount, WalletError::InsufficientBalance);
    require!(user.key() == authority.authority, WalletError::NotOwnerAllowed);

    let (_, bump) = Pubkey::find_program_address(&[MASTER_WALLET], &ctx.program_id);
    let vault_seeds = &[MASTER_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

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
        signer,
    )?;

    user_pool.sol_amount -= amount;

    Ok(())
}

pub fn withdraw_usdc_by_admin(ctx: Context<WithdrawUsdcByAdmin>, amount: u64) -> Result<()> {
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

pub fn withdraw_usdt_by_admin(ctx: Context<WithdrawUsdtByAdmin>, amount: u64) -> Result<()> {
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

pub fn withdraw_sol_by_admin(ctx: Context<WithdrawSolByAdmin>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    let destination = &accts.receiver;
    let source = &accts.master_wallet;
    let authority = &accts.config;
    let user = &accts.user;

    require!(user.key() == authority.authority, WalletError::NotOwnerAllowed);

    let (_, bump) = Pubkey::find_program_address(&[MASTER_WALLET], &ctx.program_id);
    let vault_seeds = &[MASTER_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

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
        signer,
    )?;

    Ok(())
}

pub fn withdraw_usdc_from_exchange(ctx: Context<WithdrawUsdcFromExchange>, amount: u64) -> Result<()> {
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
    let seeds = &[EXCHANGE_CONFIG, &[ctx.bumps.exchange_config]];
    let signers = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signers),
        amount,
    )?;

    Ok(())
}

pub fn withdraw_usdt_from_exchange(ctx: Context<WithdrawUsdtFromExchange>, amount: u64) -> Result<()> {
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
    let seeds = &[EXCHANGE_CONFIG, &[ctx.bumps.exchange_config]];
    let signers = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signers),
        amount,
    )?;

    Ok(())
}

pub fn withdraw_sol_from_exchange(ctx: Context<WithdrawSolFromExchange>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    let destination = &accts.receiver;
    let source = &accts.exchange_wallet;
    let authority = &accts.config;
    let user = &accts.user;

    require!(user.key() == authority.authority, WalletError::NotOwnerAllowed);

    let (_, bump) = Pubkey::find_program_address(&[EXCHANGE_WALLET], &ctx.program_id);
    let vault_seeds = &[EXCHANGE_WALLET, &[bump]];
    let signer = &[&vault_seeds[..]];

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
        signer,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct WithdrawUsdc<'info> {
    #[account(
        seeds = [CONFIG],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY)]
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
        associated_token::authority = receiver,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [USER_AUTHORITY, user_wallet_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
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
        associated_token::authority = receiver,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [USER_AUTHORITY, user_wallet_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
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

    #[account(
        mut,
        seeds = [USER_AUTHORITY, user_wallet_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawUsdtByAdmin<'info> {
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
        associated_token::authority = receiver,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSolByAdmin<'info> {
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
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawUsdcByAdmin<'info> {
    #[account(
        seeds = [CONFIG],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY)]
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
        associated_token::authority = receiver,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct WithdrawUsdtFromExchange<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [EXCHANGE_CONFIG], 
        bump
    )]
    pub exchange_config: AccountInfo<'info>,

    #[account(constraint = mint.key().to_string() == USDT_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = exchange_config
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = receiver,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSolFromExchange<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [EXCHANGE_CONFIG], 
        bump
    )]
    pub exchange_config: AccountInfo<'info>,
    
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut, 
        seeds = [EXCHANGE_WALLET], 
        bump
    )]
    pub exchange_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawUsdcFromExchange<'info> {
    #[account(
        seeds = [CONFIG], 
        bump
    )]
    pub config: Account<'info, Config>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        seeds = [EXCHANGE_CONFIG], 
        bump
    )]
    pub exchange_config: AccountInfo<'info>,

    #[account(constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = exchange_config
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = receiver,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
