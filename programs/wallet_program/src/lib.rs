pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("HbHtUUTSrfqJ5BGBC7ydnTRqirh61nYdVe9KYvgPh1G9");

#[program]
pub mod wallet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        instructions::initialize(ctx, bump)
    }
    pub fn initialize_usdc(ctx: Context<InitializeUsdc>) -> Result<()> {
        instructions::initialize_usdc(ctx)
    }

    pub fn deposit_usdc(ctx: Context<DepositUsdc>,user_wallet_index: u32, amount: u64) -> Result<()> {
        instructions::deposit_usdc(ctx,user_wallet_index, amount)
    }

    pub fn deposit_usdt(ctx: Context<DepositUsdt>,user_wallet_index: u32, amount: u64) -> Result<()> {
        instructions::deposit_usdt(ctx, user_wallet_index, amount)
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, user_wallet_index: u32,amount: u64) -> Result<()> {
        instructions::deposit_sol(ctx, user_wallet_index, amount)
    }

    pub fn batch_withdraw(ctx: Context<BatchWithdraw>, amount: Vec<u64>) -> Result<()> {
        instructions::batch_withdraw(ctx, amount)
    }

    pub fn forward_usdc_to_admin(ctx: Context<ForwardUsdcToAdmin>,  user_wallet_index: u32) ->  Result<()> {
        instructions::forward_usdc_to_admin(ctx, user_wallet_index)
    }

    pub fn forward_usdt_to_admin(ctx: Context<ForwardUsdTtoAdmin>,  user_wallet_index: u32) ->  Result<()> {
        instructions::forward_usdt_to_admin(ctx, user_wallet_index)
    }

    pub fn forward_sol_to_admin(ctx: Context<ForwardSolToAdmin>,  user_wallet_index: u32, amount: u64) ->  Result<()> {
        instructions::forward_sol_to_admin(ctx, user_wallet_index, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>,amount: u64) ->  Result<()> {
        instructions::withdraw(ctx, amount)
    }
    pub fn withdraw_usdt(ctx: Context<WithdrawUsdt>,amount: u64) ->  Result<()> {
        instructions::withdraw_usdt(ctx, amount)
    }
    pub fn withdraw_sol(ctx: Context<WithdrawSol>,amount: u64) ->  Result<()> {
        instructions::withdraw_sol(ctx, amount)
    }
}
