use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Program id not match")]
    ProgramIdNotMatch,

    #[msg("Remaining accounts not match")]
    RemainingAccountsNotMatch,

    #[msg("Admin not match")]
    AdminNotMatch,

    #[msg("Pending admin not match")]
    PendingAdminNotMatch,

    #[msg("Stake manager already exist")]
    StakeManagerAlreadyExist,

    #[msg("Stake manager not exist")]
    StakeManagerNotExist,
}
