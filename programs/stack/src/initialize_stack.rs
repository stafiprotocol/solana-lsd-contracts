pub use crate::errors::Errors;
use crate::Stack;
use anchor_lang::prelude::*;

pub const STACK_SEED: &'static [u8] = b"stack_seed";
#[derive(Accounts)]
#[instruction(stack_index: u8)]
pub struct InitializeStack<'info> {
    #[account(
        init,
        space = 1024,
        payer = rent_payer,
        rent_exempt = enforce,
        seeds = [
            STACK_SEED,
            &admin.key().to_bytes(),
            &[stack_index],
        ],
        bump,
    )]
    pub stack: Box<Account<'info, Stack>>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeStack<'info> {
    pub fn process(&mut self, _stack_index: u8) -> Result<()> {
        self.stack.set_inner(Stack {
            admin: self.admin.key(),
            pending_admin: Pubkey::default(),
            stack_fee_commission: Stack::DEFAULT_STACK_FEE_COMMISSION,
            stake_managers_len_limit: Stack::DEFAULT_STAKE_MANAGERS_LEN_LIMIT,
            entrusted_stake_managers: vec![],
        });

        Ok(())
    }
}
