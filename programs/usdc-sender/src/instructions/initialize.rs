use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32, 
        seeds = [
            b"global-authority".as_ref(),
        ],
        bump,
    )]
    pub global_pool: Account<'info, GlobalPool>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let global_pool = &mut ctx.accounts.global_pool;

    global_pool.admin = *ctx.accounts.admin.key;

    Ok(())
}