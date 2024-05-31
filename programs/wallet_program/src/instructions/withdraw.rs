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

pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.config;
    let user_pool = &mut ctx.accounts.user_pool;

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

    user_pool.credit_amount -= amount;

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

#[derive(Accounts)]
pub struct WithdrawUsdc<'info> {
    #[account(
        seeds = [CONFIG],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [USER_AUTHORITY, user.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = config
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = user,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
