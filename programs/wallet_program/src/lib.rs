pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("FbhjncVYSiKZue8XMZpRYcCR7FTsh2TpwHomBGm3NQ2");

#[program]
pub mod wallet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        instructions::initialize(ctx, bump)
    }
    pub fn initialize_usdc(ctx: Context<InitializeUsdc>) -> Result<()> {
        instructions::initialize_usdc(ctx)
    }

    pub fn batch_withdraw(ctx: Context<BatchWithdraw>, amount: Vec<u64>) -> Result<()> {
        instructions::batch_withdraw(ctx, amount)
    }

    pub fn forward_to_admin(ctx: Context<ForwardToAdmin>,  user_wallet_index: u32) ->  Result<()> {
        instructions::forward_to_admin(ctx, user_wallet_index)
    }

    pub fn withdraw(ctx: Context<Withdraw>, user_wallet: u8,amount: u64) ->  Result<()> {
        instructions::withdraw(ctx, user_wallet, amount)
    }
}
