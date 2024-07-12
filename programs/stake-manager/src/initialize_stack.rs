pub use crate::errors::Errors;
use crate::Stack;
pub use crate::StakeManager;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeStack<'info> {
    #[account(
        zero,
        rent_exempt = enforce,
    )]
    pub stack: Box<Account<'info, Stack>>,

    pub admin: Signer<'info>,
}

impl<'info> InitializeStack<'info> {
    pub fn process(&mut self) -> Result<()> {
        self.stack.set_inner(Stack {
            admin: self.admin.key(),
            stack_fee_commission: Stack::DEFAULT_STACK_FEE_COMMISSION,
            entrusted_stake_managers: vec![],
        });

        Ok(())
    }
}
