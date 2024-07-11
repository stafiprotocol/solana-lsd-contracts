use crate::{Errors, Stack};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferStackAdmin<'info> {
    #[account(
        mut, 
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stack: Box<Account<'info, Stack>>,

    pub admin: Signer<'info>,
}

impl<'info> TransferStackAdmin<'info> {
    pub fn process(&mut self, new_admin: Pubkey) -> Result<()> {
        self.stack.admin = new_admin;

        msg!("TransferStackAdmin: new admin: {}", new_admin);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetStackFeeCommission<'info> {
    #[account(
        mut, 
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stack: Box<Account<'info, Stack>>,

    pub admin: Signer<'info>,
}

impl<'info> SetStackFeeCommission<'info> {
    pub fn process(&mut self, stack_fee_commission: u64) -> Result<()> {
        self.stack.stack_fee_commission = stack_fee_commission;

        msg!("SetStackFeeCommission: {}", stack_fee_commission);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddEntrustedStakeManager<'info> {
    #[account(
        mut, 
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stack: Box<Account<'info, Stack>>,

    pub admin: Signer<'info>,
}

impl<'info> AddEntrustedStakeManager<'info> {
    pub fn process(&mut self, stake_manager: Pubkey) -> Result<()> {
        require!(
            !self.stack.entrusted_stake_managers.contains(&stake_manager),
            Errors::StakeManagerAlreadyExist
        );

        self.stack.entrusted_stake_managers.push(stake_manager);

        msg!(
            "AddEntrustedStakeManager: {}",
            stake_manager.key().to_string()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RemoveEntrustedStakeManager<'info> {
    #[account(
        mut, 
        has_one = admin @ Errors::AdminNotMatch
    )]
    pub stack: Box<Account<'info, Stack>>,

    pub admin: Signer<'info>,
}

impl<'info> RemoveEntrustedStakeManager<'info> {
    pub fn process(&mut self, stake_manager: Pubkey) -> Result<()> {
        require!(
            self.stack.entrusted_stake_managers.contains(&stake_manager),
            Errors::ValidatorNotExist
        );

        self.stack
            .entrusted_stake_managers
            .retain(|&e| e != stake_manager);

        msg!(
            "RemoveEntrustedStakeManager: {}",
            stake_manager.key().to_string()
        );
        Ok(())
    }
}
