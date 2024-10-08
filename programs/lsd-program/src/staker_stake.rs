use crate::{Errors, StakeManager};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut,
        has_one = lsd_token_mint @Errors::MintAccountNotMatch,
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    #[account(
        mut,
        seeds = [
            &stake_manager.key().to_bytes(),
            StakeManager::POOL_SEED,
        ],
        bump = stake_manager.pool_seed_bump
    )]
    pub stake_pool: SystemAccount<'info>,

    #[account(
        mut,
        owner = system_program::ID,
        address = mint_to.owner @ Errors::MintToOwnerNotMatch
    )]
    pub from: Signer<'info>,

    #[account(mut)]
    pub lsd_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = stake_manager.lsd_token_mint
    )]
    pub mint_to: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct EventStake {
    pub era: u64,
    pub staker: Pubkey,
    pub mint_to: Pubkey,
    pub stake_amount: u64,
    pub lsd_token_amount: u64,
}

impl<'info> Stake<'info> {
    pub fn process(&mut self, stake_amount: u64) -> Result<()> {
        require_gte!(
            stake_amount,
            self.stake_manager.min_stake_amount,
            Errors::StakeAmountTooLow
        );

        let user_balance = self.from.lamports();
        require_gte!(user_balance, stake_amount, Errors::BalanceNotEnough);

        let lsd_token_amount = self.stake_manager.calc_lsd_token_amount(stake_amount)?;

        self.stake_manager.era_bond += stake_amount;
        self.stake_manager.active += stake_amount;

        // transfer lamports to the pool
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.from.to_account_info(),
                    to: self.stake_pool.to_account_info(),
                },
            ),
            stake_amount,
        )?;

        // mint lsd token
        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.lsd_token_mint.to_account_info(),
                    to: self.mint_to.to_account_info(),
                    authority: self.stake_pool.to_account_info(),
                },
                &[&[
                    &self.stake_manager.key().to_bytes(),
                    StakeManager::POOL_SEED,
                    &[self.stake_manager.pool_seed_bump],
                ]],
            ),
            lsd_token_amount,
        )?;

        emit!(EventStake {
            era: self.stake_manager.latest_era,
            staker: self.from.key(),
            mint_to: self.mint_to.key(),
            stake_amount,
            lsd_token_amount
        });
        Ok(())
    }
}
