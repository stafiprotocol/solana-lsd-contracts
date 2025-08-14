use crate::{Errors, StakeManager};
use anchor_lang::{prelude::*, system_program};
use stack::Stack;
#[derive(Accounts)]
pub struct TransferStakeManagerAdmin<'info> {
    #[account(
        mut,
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub admin: Signer<'info>,
}

impl<'info> TransferStakeManagerAdmin<'info> {
    pub fn process(&mut self, new_admin: Pubkey) -> Result<()> {
        self.stake_manager.pending_admin = new_admin;

        msg!("TransferStakeManagerAdmin: new admin: {}", new_admin);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AcceptStakeManagerAdmin<'info> {
    pub pending_admin: Signer<'info>,

    #[account(
        mut,
        has_one = pending_admin @ Errors::PendingAdminNotMatch
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,
}

impl<'info> AcceptStakeManagerAdmin<'info> {
    pub fn process(&mut self) -> Result<()> {
        self.stake_manager.admin = self.stake_manager.pending_admin;
        self.stake_manager.pending_admin = Pubkey::default();

        msg!("AcceptAdmin: {}", self.stake_manager.admin);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetPlatformStackFeeCommission<'info> {
    #[account(
        mut,
        has_one = stack @ Errors::StackNotMatch,
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    #[account(
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stack: Box<Account<'info, Stack>>,

    pub admin: Signer<'info>,
}

impl<'info> SetPlatformStackFeeCommission<'info> {
    pub fn process(&mut self, stack_fee_commission: u64) -> Result<()> {
        self.stake_manager.stack_fee_commission = stack_fee_commission;

        msg!("SetPlatformStackFeeCommission: {}", stack_fee_commission);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(new_size: u32)]
pub struct ReallocStakeManager<'info> {
    #[account(
        mut,
        has_one = admin @ Errors::AdminNotMatch,
        realloc = new_size as usize,
        realloc::payer = rent_payer,
        realloc::zero = false,
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub admin: Signer<'info>,

    #[account(
        mut,
        owner = system_program::ID,
    )]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ReallocStakeManager<'info> {
    pub fn process(&mut self, new_size: u32) -> Result<()> {
        msg!("new_size {}", new_size);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigStakeManager<'info> {
    #[account(
        mut,
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub admin: Signer<'info>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct ConfigStakeManagerParams {
    pub min_stake_amount: Option<u64>,
    pub platform_fee_commission: Option<u64>,
    pub unbonding_duration: Option<u64>,
    pub rate_change_limit: Option<u64>,
    pub balancer: Option<Pubkey>,
    pub add_validator: Option<Pubkey>,
    pub remove_validator: Option<Pubkey>,
}

impl<'info> ConfigStakeManager<'info> {
    pub fn process(&mut self, config_stake_manager_params: ConfigStakeManagerParams) -> Result<()> {
        if let Some(min_stake_amount) = config_stake_manager_params.min_stake_amount {
            self.stake_manager.min_stake_amount = min_stake_amount;
            msg!("min_stake_amount: {}", min_stake_amount);
        }

        if let Some(platform_fee_commission) = config_stake_manager_params.platform_fee_commission {
            require!(
                platform_fee_commission < 1_000_000_000,
                Errors::ParamsNotMatch
            );

            self.stake_manager.platform_fee_commission = platform_fee_commission;
            msg!("platform_fee_commission: {}", platform_fee_commission);
        }

        if let Some(rate_change_limit) = config_stake_manager_params.rate_change_limit {
            self.stake_manager.rate_change_limit = rate_change_limit;
            msg!("rate_change_limit: {}", rate_change_limit);
        }

        if let Some(balancer) = config_stake_manager_params.balancer {
            self.stake_manager.balancer = balancer;
            msg!("balancer: {}", balancer);
        }

        if let Some(add_validator) = config_stake_manager_params.add_validator {
            require!(
                !self.stake_manager.validators.contains(&add_validator),
                Errors::ValidatorAlreadyExist
            );

            self.stake_manager.validators.push(add_validator);

            msg!(
                "AddValidator: new validator: {}",
                add_validator.key().to_string()
            );
        }

        if let Some(remove_validator) = config_stake_manager_params.remove_validator {
            require!(
                self.stake_manager.validators.contains(&remove_validator),
                Errors::ValidatorNotExist
            );

            self.stake_manager
                .validators
                .retain(|&e| e != remove_validator);

            msg!(
                "RemoveValidator: remove validator: {}",
                remove_validator.key().to_string()
            );
        }

        Ok(())
    }
}
