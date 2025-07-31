pub use crate::errors::Errors;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct Stack {
    pub admin: Pubkey,
    pub pending_admin: Pubkey,
    pub stack_fee_commission: u64, // decimals 9
    pub stake_managers_len_limit: u64,
    pub entrusted_stake_managers: Vec<Pubkey>,
}

impl Stack {
    pub const DEFAULT_STACK_FEE_COMMISSION: u64 = 100_000_000;
    pub const DEFAULT_STAKE_MANAGERS_LEN_LIMIT: u64 = 20;
}
