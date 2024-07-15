pub use crate::errors::Errors;
use crate::Stack;
pub use crate::StakeManager;
use anchor_lang::{prelude::*, solana_program::system_program};

#[derive(Accounts)]
pub struct InitializeStack<'info> {
    #[account(
        init,
        space = 1000,
        payer = rent_payer,
        rent_exempt = enforce,
    )]
    pub stack: Box<Account<'info, Stack>>,

    #[account(
        mut,
        owner = system_program::ID
    )]
    pub rent_payer: Signer<'info>,

    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeStack<'info> {
    pub fn process(&mut self) -> Result<()> {
        self.stack.set_inner(Stack {
            admin: self.admin.key(),
            stack_fee_commission: Stack::DEFAULT_STACK_FEE_COMMISSION,
            stake_managers_len_limit: Stack::DEFAULT_STAKE_MANAGERS_LEN_LIMIT,
            entrusted_stake_managers: vec![],
        });

        Ok(())
    }
}
