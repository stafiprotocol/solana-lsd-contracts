use crate::{Errors, Stack, StackFeeAccount, StakeManager};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct EraUpdateRate<'info> {
    #[account(
        mut, 
        has_one = stack @ Errors::StackNotMatch,
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub stack: Box<Account<'info, Stack>>,

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
        associated_token::mint = lsd_token_mint,
        associated_token::authority = stake_manager.admin,
    )]
    pub platform_fee_recipient: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = lsd_token_mint,
        associated_token::authority = stack.admin,
    )]
    pub stack_fee_recipient: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            &stake_manager.key().to_bytes(),
            &lsd_token_mint.key().to_bytes(),
        ],
        bump = stack_fee_account.bump,
    )]
    pub stack_fee_account: Box<Account<'info, StackFeeAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct EventEraUpdateRate {
    pub era: u64,
    pub rate: u64,
    pub platform_fee: u64,
    pub stack_fee: u64,
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

        let platform_fee_raw = self.stake_manager.calc_platform_fee(reward)?;
        let stack_fee = self.stack.calc_stack_fee(platform_fee_raw)?;
        let platform_fee = platform_fee_raw - stack_fee;

        if platform_fee > 0 {
            mint_to(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.lsd_token_mint.to_account_info(),
                        to: self.platform_fee_recipient.to_account_info(),
                        authority: self.stake_pool.to_account_info(),
                    },
                    &[&[
                        &self.stake_manager.key().to_bytes(),
                        StakeManager::POOL_SEED,
                        &[self.stake_manager.pool_seed_bump],
                    ]],
                ),
                platform_fee,
            )?;

            self.stake_manager.total_platform_fee += platform_fee;
        }
        if stack_fee > 0 {
            mint_to(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.lsd_token_mint.to_account_info(),
                        to: self.stack_fee_recipient.to_account_info(),
                        authority: self.stake_pool.to_account_info(),
                    },
                    &[&[
                        &self.stake_manager.key().to_bytes(),
                        StakeManager::POOL_SEED,
                        &[self.stake_manager.pool_seed_bump],
                    ]],
                ),
                stack_fee,
            )?;

            self.stack_fee_account.amount += stack_fee;
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
            platform_fee: platform_fee,
            stack_fee: stack_fee,
        });
        Ok(())
    }
}
