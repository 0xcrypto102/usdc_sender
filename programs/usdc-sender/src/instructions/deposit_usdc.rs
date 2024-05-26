use {
    crate::state::*,
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
    },
};

#[derive(Accounts)]
pub struct DepositUsdc<'info> {
    #[account(
        seeds = [b"global-authority".as_ref()],
        bump
    )]
    pub global_pool: Account<'info, GlobalPool>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [b"user-authority".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserPool>,

    #[account(constraint = mint.key().to_string() == USDC_TOKEN_MINT_PUBKEY)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = global_pool,
        payer = user,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DepositUsdc>, amount: u64) -> Result<()> {
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

    Ok(())
}
