use anchor_lang::{prelude::*, Bumps};

pub mod admin_stake_manager;
pub mod era_bond;
pub mod era_merge;
pub mod era_new;
pub mod era_skip_bond;
pub mod era_unbond;
pub mod era_update_active;
pub mod era_update_rate;
pub mod era_withdraw;
pub mod errors;
pub mod helper;
pub mod initialize_stake_manager;
pub mod metadata;
pub mod redelegate;
pub mod staker_stake;
pub mod staker_unstake;
pub mod staker_withdraw;
pub mod states;

pub use crate::admin_stake_manager::*;
pub use crate::era_bond::*;
pub use crate::era_merge::*;
pub use crate::era_new::*;
pub use crate::era_skip_bond::*;
pub use crate::era_unbond::*;
pub use crate::era_update_active::*;
pub use crate::era_update_rate::*;
pub use crate::era_withdraw::*;
pub use crate::errors::Errors;
pub use crate::helper::*;
pub use crate::initialize_stake_manager::*;
pub use crate::metadata::*;
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
pub mod stake_manager {

    use super::*;

    // initialize account

    pub fn initialize_stake_manager(
        ctx: Context<InitializeStakeManager>,
        stake_manager_index: u8,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(
            stake_manager_index,
            ctx.bumps.stake_pool,
            ctx.bumps.stack_fee_account,
        )?;

        Ok(())
    }

    // admin of stack

    pub fn set_platform_stack_fee_commission(
        ctx: Context<SetPlatformStackFeeCommission>,
        stack_fee_commission: u64,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stack_fee_commission)?;

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

    pub fn accept_stake_manager_admin(ctx: Context<AcceptStakeManagerAdmin>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

        Ok(())
    }

    pub fn config_stake_manager(
        ctx: Context<ConfigStakeManager>,
        params: ConfigStakeManagerParams,
    ) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(params)?;

        Ok(())
    }

    pub fn realloc_stake_manager(ctx: Context<ReallocStakeManager>, new_size: u32) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_size)?;

        Ok(())
    }

    // metadata
    pub fn create_metadata(
        ctx: Context<CreateMetadataV1>,
        params: CreateMetadataParams,
    ) -> Result<()> {
        check_context(&ctx)?;
        ctx.accounts.process(params)
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

    pub fn era_skip_bond(ctx: Context<EraSkipBond>) -> Result<()> {
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
