use {
    crate::{
        constants::*, state::*
    },
    anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
    }, std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(user_wallet_index: u32)]
pub struct DepositUsdc<'info> {
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
    pub user_pool: Box<Account<'info, UserPool>>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [USER_WALLET, user_wallet_index.to_le_bytes().as_ref()], 
        bump
    )]
    pub user_wallet: AccountInfo<'info>,

    #[account(
        mut,
        constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user
    )]
    pub from_ata: Account<'info, TokenAccount>,

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
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_usdc(ctx: Context<DepositUsdc>,user_wallet_index: u32, amount: u64) -> Result<()> {
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

    ctx.accounts.user_pool.credit_amount += amount;
    ctx.accounts.user_pool.usdc_amount += amount;

    Ok(())
}
