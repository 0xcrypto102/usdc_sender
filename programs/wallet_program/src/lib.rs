pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("AqTPKZr4jsWKHPtUWBd2gScwQi2Narducnc77SggATdR");

#[program]
pub mod wallet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        instructions::initialize(ctx, bump)
    }
    
    pub fn initialize_usdc(ctx: Context<InitializeUsdc>) -> Result<()> {
        instructions::initialize_usdc(ctx)
    }

    pub fn initialize_exchange(ctx: Context<InitializeExchange>) -> Result<()> {
        instructions::initialize_exchange(ctx)
    }
    
    pub fn initialize_exchange_usdc(ctx: Context<InitializeExchangeUsdc>) -> Result<()> {
        instructions::initialize_exchange_usdc(ctx)
    }

    pub fn initialize_user_usdc_wallet(ctx: Context<InitializeUserUsdcWallet>, user_wallet_index: u32) -> Result<()> {
        instructions::initialize_user_usdc_wallet(ctx, user_wallet_index)
    }

    pub fn initialize_user_usdt_wallet(ctx: Context<InitializeUserUsdtWallet>, user_wallet_index: u32) -> Result<()> {
        instructions::initialize_user_usdt_wallet(ctx, user_wallet_index)
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

    pub fn trasnfer_usdt_to_exchange(ctx: Context<TransferUsdtToExchange>, amount: u64) -> Result<()> {
        instructions::trasnfer_usdt_to_exchange(ctx, amount)
    }

    pub fn trasnfer_usdc_to_exchange(ctx: Context<TransferUsdcToExchange>, amount: u64) -> Result<()> {
        instructions::trasnfer_usdc_to_exchange(ctx, amount)
    }

    pub fn transfer_sol_to_exchange(ctx: Context<TransferSolToExchange>, amount: u64) -> Result<()> {
        instructions::transfer_sol_to_exchange(ctx, amount)
    }

    pub fn trasnfer_usdt_to_master(ctx: Context<TransferUsdcToMaster>, amount: u64) -> Result<()> {
        instructions::trasnfer_usdt_to_master(ctx, amount)
    }

    pub fn trasnfer_usdc_to_master(ctx: Context<TransferUsdtToMaster>, amount: u64) -> Result<()> {
        instructions::trasnfer_usdc_to_master(ctx, amount)
    }

    pub fn transfer_sol_to_master(ctx: Context<TransferSolToMaster>, amount: u64) -> Result<()> {
        instructions::transfer_sol_to_master(ctx, amount)
    }

    pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>,user_wallet_index: u32,amount: u64) ->  Result<()> {
        instructions::withdraw_usdc(ctx,user_wallet_index, amount)
    }

    pub fn withdraw_usdt(ctx: Context<WithdrawUsdt>,user_wallet_index: u32,amount: u64) ->  Result<()> {
        instructions::withdraw_usdt(ctx,user_wallet_index, amount)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, user_wallet_index: u32,amount: u64) ->  Result<()> {
        instructions::withdraw_sol(ctx,user_wallet_index, amount)
    }

    pub fn withdraw_usdc_by_admin(ctx: Context<WithdrawUsdcByAdmin>, amount: u64) ->  Result<()> {
        instructions::withdraw_usdc_by_admin(ctx, amount)
    }

    pub fn withdraw_usdt_by_admin(ctx: Context<WithdrawUsdtByAdmin>, amount: u64) ->  Result<()> {
        instructions::withdraw_usdt_by_admin(ctx, amount)
    }

    pub fn withdraw_sol_by_admin(ctx: Context<WithdrawSolByAdmin>,amount: u64) ->  Result<()> {
        instructions::withdraw_sol_by_admin(ctx,amount)
    }

    pub fn withdraw_usdc_from_exchange(ctx: Context<WithdrawUsdcFromExchange>, amount: u64) ->  Result<()> {
        instructions::withdraw_usdc_from_exchange(ctx, amount)
    }

    pub fn withdraw_usdt_from_exchange(ctx: Context<WithdrawUsdtFromExchange>, amount: u64) ->  Result<()> {
        instructions::withdraw_usdt_from_exchange(ctx, amount)
    }

    pub fn withdraw_sol_from_exchange(ctx: Context<WithdrawSolFromExchange>,amount: u64) ->  Result<()> {
        instructions::withdraw_sol_from_exchange(ctx,amount)
    }
}
