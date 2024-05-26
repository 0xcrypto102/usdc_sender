use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("7pB3zj3voieg4msvT7oTxc6xRDDPDjiCy3J4Sr4ogPaC");

#[program]
pub mod usdc_sender {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn deposit(ctx: Context<DepositUsdc>, amount: u64) -> Result<()> {
        deposit_usdc::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<WithdrawUsdc>, amount: u64) -> Result<()> {
        withdraw_usdc::handler(ctx, amount)
    }

    pub fn withdraw_admin(ctx: Context<WithdrawAdmin>, amount: u64) -> Result<()> {
        withdraw_admin::handler(ctx, amount)
    }
}
