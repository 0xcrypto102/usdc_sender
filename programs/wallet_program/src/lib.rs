pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("RB57sATNhML6QK289L8gmLekytJxgzwxFS8UN2WsnWB");

#[program]
pub mod wallet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        instructions::initialize(ctx, bump)
    }
 

    pub fn initialize_user_wallet(ctx: Context<InitializeUserWallet>, user_wallet_index: u32) -> Result<()> {
        instructions::initialize_user_wallet(ctx, user_wallet_index)
    }

    pub fn batch_withdraw(ctx: Context<BatchWithdraw>, amount: Vec<u64>) -> Result<()> {
        instructions::batch_withdraw(ctx, amount)
    }

    pub fn forward_usdt_to_admin(ctx: Context<ForwardUsdTtoAdmin>,  user_wallet_index: u32) ->  Result<()> {
        instructions::forward_usdt_to_admin(ctx, user_wallet_index)
    }

    pub fn forward_sol_to_admin(ctx: Context<ForwardSolToAdmin>,  user_wallet_index: u32, amount: u64) ->  Result<()> {
        instructions::forward_sol_to_admin(ctx, user_wallet_index, amount)
    }

    pub fn withdraw_usdt(ctx: Context<WithdrawUsdt>,user_wallet_index: u32,amount: u64) ->  Result<()> {
        instructions::withdraw_usdt(ctx,user_wallet_index, amount)
    }
    pub fn withdraw_sol(ctx: Context<WithdrawSol>, user_wallet_index: u32,amount: u64) ->  Result<()> {
        instructions::withdraw_sol(ctx,user_wallet_index, amount)
    }
}
