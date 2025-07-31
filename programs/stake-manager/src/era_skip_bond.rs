use crate::{Errors, StakeManager};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake::tools;
use anchor_spl::stake::Stake;

#[derive(Accounts)]
pub struct EraSkipBond<'info> {
    #[account(mut)]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub stake_program: Program<'info, Stake>,
}

#[event]
pub struct EventEraSkipBond {
    pub era: u64,
    pub skip_bond_amount: u64,
}

impl<'info> EraSkipBond<'info> {
    pub fn process(&mut self) -> Result<()> {
        require!(
            self.stake_manager
                .era_process_data
                .need_skip_bond(tools::get_minimum_delegation()?),
            Errors::EraNoNeedSkipBond
        );

        let need_bond = self.stake_manager.era_process_data.need_bond;

        self.stake_manager.era_bond += need_bond;
        self.stake_manager.era_process_data.new_active += need_bond;
        self.stake_manager.era_process_data.need_bond = 0;

        emit!(EventEraSkipBond {
            era: self.stake_manager.latest_era,
            skip_bond_amount: need_bond
        });
        Ok(())
    }
}
