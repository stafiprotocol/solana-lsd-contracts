use anchor_lang::{prelude::*, Bumps};

pub mod admin_stack;
pub mod errors;
pub mod initialize_stack;
pub mod states;

pub use crate::admin_stack::*;
pub use crate::errors::Errors;
pub use crate::initialize_stack::*;
pub use crate::states::*;

declare_id!("Gr8cqDNLLAQ5DvYHGRijfhYVfFZD3KK6oV6Uyu2qVHXB");

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
pub mod stack {

    use super::*;

    // initialize account

    pub fn initialize_stack(ctx: Context<InitializeStack>, stack_index: u8) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(stack_index)?;

        Ok(())
    }

    // admin of stack

    pub fn transfer_stack_admin(ctx: Context<TransferStackAdmin>, new_admin: Pubkey) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process(new_admin)?;

        Ok(())
    }

    pub fn accept_stack_admin(ctx: Context<AcceptStackAdmin>) -> Result<()> {
        check_context(&ctx)?;

        ctx.accounts.process()?;

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
}
