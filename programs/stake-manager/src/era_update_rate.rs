use crate::{Errors, StakeManager};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct EraUpdateRate<'info> {
    #[account(
        mut, 
        has_one = fee_recipient @ Errors::FeeRecipientNotMatch
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    #[account(
        seeds = [
            &stake_manager.key().to_bytes(),
            StakeManager::POOL_SEED,
        ],
        bump = stake_manager.pool_seed_bump
    )]
    pub stake_pool: SystemAccount<'info>,

    #[account(mut)]
    pub lsd_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = stake_manager.lsd_token_mint
    )]
    pub fee_recipient: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

#[event]
pub struct EventEraUpdateRate {
    pub era: u64,
    pub rate: u64,
    pub fee: u64,
}

impl<'info> EraUpdateRate<'info> {
    pub fn process(&mut self) -> Result<()> {
        require!(
            self.stake_manager.era_process_data.need_update_rate(),
            Errors::EraNoNeedUpdateRate
        );

        let reward = if self.stake_manager.era_process_data.new_active
            > self.stake_manager.era_process_data.old_active
        {
            self.stake_manager.era_process_data.new_active
                - self.stake_manager.era_process_data.old_active
        } else {
            0
        };

        let protocol_fee = self.stake_manager.calc_protocol_fee(reward)?;
        if protocol_fee > 0 {
            mint_to(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.lsd_token_mint.to_account_info(),
                        to: self.fee_recipient.to_account_info(),
                        authority: self.stake_pool.to_account_info(),
                    },
                    &[&[
                        &self.stake_manager.key().to_bytes(),
                        StakeManager::POOL_SEED,
                        &[self.stake_manager.pool_seed_bump],
                    ]],
                ),
                protocol_fee,
            )?;

            self.stake_manager.total_protocol_fee += protocol_fee;
        }

        let cal_temp = self.stake_manager.active + self.stake_manager.era_process_data.new_active;
        let new_active = if cal_temp > self.stake_manager.era_process_data.old_active {
            cal_temp - self.stake_manager.era_process_data.old_active
        } else {
            0
        };

        let new_rate = self
            .stake_manager
            .calc_rate(new_active, self.lsd_token_mint.supply)?;
        let rate_change = self
            .stake_manager
            .calc_rate_change(self.stake_manager.rate, new_rate)?;
        require_gte!(
            self.stake_manager.rate_change_limit,
            rate_change,
            Errors::RateChangeOverLimit
        );

        self.stake_manager.era_process_data.old_active = 0;
        self.stake_manager.era_process_data.new_active = 0;
        self.stake_manager.active = new_active;
        self.stake_manager.rate = new_rate;

        emit!(EventEraUpdateRate {
            era: self.stake_manager.latest_era,
            rate: new_rate,
            fee: protocol_fee
        });
        Ok(())
    }
}
