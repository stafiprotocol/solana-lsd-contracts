use anchor_lang::{prelude::*, Bumps};

pub mod admin_stack;
pub mod admin_stake_manager;
pub mod era_bond;
pub mod era_merge;
pub mod era_new;
pub mod era_unbond;
pub mod era_update_active;
pub mod era_update_rate;
pub mod era_withdraw;
pub mod errors;
pub mod initialize_stack;
pub mod initialize_stake_manager;
pub mod redelegate;
pub mod staker_stake;
pub mod staker_unstake;
pub mod staker_withdraw;
pub mod states;

pub use crate::admin_stack::*;
pub use crate::admin_stake_manager::*;
pub use crate::era_bond::*;
pub use crate::era_merge::*;
pub use crate::era_new::*;
pub use crate::era_unbond::*;
pub use crate::era_update_active::*;
pub use crate::era_update_rate::*;
pub use crate::era_withdraw::*;
pub use crate::errors::Errors;
pub use crate::initialize_stack::*;
pub use crate::initialize_stake_manager::*;
pub use crate::redelegate::*;
pub use crate::staker_stake::*;
pub use crate::staker_unstake::*;
pub use crate::staker_withdraw::*;
pub use crate::states::*;

declare_id!("795MBfkwwtAX4fWiFqZcJK8D91P9tqqtiSRrSNhBvGzq");

fn check_context<T: Bumps>(ctx: &Context<T>) -> Result<()> {
    if !check_id(ctx.program_id) {
        return err!(Errors::ProgramIdNotMatch);
    }

    if !ctx.remaining_accounts.is_empty() {
        return err!(Errors::RemainingAccountsNotMatch);
    }

    Ok(())
}

#[program]
pub mod stake_manager_program {

    use super::*;

    // initialize account

    pub fn initialize_stack(ctx: Context<InitializeStack>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn initialize_stake_manager(ctx: Context<InitializeStakeManager>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts
            .process(ctx.bumps.stake_pool, ctx.bumps.stack_fee_account)?;

        Ok(())
    }

    // admin of stack

    pub fn transfer_stack_admin(ctx: Context<TransferStackAdmin>, new_admin: Pubkey) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_admin)?;

        Ok(())
    }

    pub fn set_stack_fee_commission(
        ctx: Context<SetStackFeeCommission>,
        stack_fee_commission: u64,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stack_fee_commission)?;

        Ok(())
    }

    pub fn set_platform_stack_fee_commission(
        ctx: Context<SetPlatformStackFeeCommission>,
        stack_fee_commission: u64,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stack_fee_commission)?;

        Ok(())
    }

    pub fn add_entrusted_stake_manager(
        ctx: Context<AddEntrustedStakeManager>,
        stake_manager: Pubkey,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stake_manager)?;

        Ok(())
    }

    pub fn remove_entrusted_stake_manager(
        ctx: Context<RemoveEntrustedStakeManager>,
        stake_manager: Pubkey,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stake_manager)?;

        Ok(())
    }

    // admin of stake manager

    pub fn transfer_stake_manager_admin(
        ctx: Context<TransferStakeManagerAdmin>,
        new_admin: Pubkey,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_admin)?;

        Ok(())
    }

    pub fn transfer_balancer(ctx: Context<TransferBalancer>, new_balancer: Pubkey) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_balancer)?;

        Ok(())
    }

    pub fn set_min_stake_amount(ctx: Context<SetMinStakeAmount>, amount: u64) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(amount)?;

        Ok(())
    }

    pub fn set_unbonding_duration(ctx: Context<SetUnbondingDuration>, duration: u64) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(duration)?;

        Ok(())
    }

    pub fn set_rate_change_limit(
        ctx: Context<SetRateChangeLimit>,
        rate_change_limit: u64,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(rate_change_limit)?;

        Ok(())
    }

    pub fn set_platform_fee_commission(
        ctx: Context<SetPlatformFeeCommission>,
        protocol_fee_commission: u64,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(protocol_fee_commission)?;

        Ok(())
    }

    pub fn add_validator(ctx: Context<AddValidator>, new_validator: Pubkey) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_validator)?;

        Ok(())
    }

    pub fn remove_validator(ctx: Context<RemoveValidator>, remove_validator: Pubkey) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(remove_validator)?;

        Ok(())
    }

    pub fn realloc_stake_manager(ctx: Context<ReallocStakeManager>, new_size: u32) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_size)?;

        Ok(())
    }

    // balancer

    pub fn redelegate(ctx: Context<Redelegate>, redelegate_amount: u64) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(redelegate_amount)?;

        Ok(())
    }

    // staker

    pub fn stake(ctx: Context<Stake>, stake_amount: u64) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stake_amount)?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, unstake_amount: u64) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(unstake_amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    // era

    pub fn era_new(ctx: Context<EraNew>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn era_bond(ctx: Context<EraBond>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn era_unbond(ctx: Context<EraUnbond>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn era_update_active(ctx: Context<EraUpdateActive>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn era_update_rate(ctx: Context<EraUpdateRate>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn era_merge(ctx: Context<EraMerge>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn era_withdraw(ctx: Context<EraWithdraw>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }
}
