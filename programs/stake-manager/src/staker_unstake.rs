use crate::{Errors, StakeManager, UnstakeAccount};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{burn, Burn, Mint, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        has_one = lsd_token_mint @ Errors::MintAccountNotMatch
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    #[account(mut)]
    pub lsd_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        token::mint = stake_manager.lsd_token_mint,
    )]
    pub burn_lsd_token_from: Box<InterfaceAccount<'info, TokenAccount>>,

    pub burn_lsd_token_authority: Signer<'info>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<UnstakeAccount>(),
        payer = rent_payer,
        rent_exempt = enforce,
    )]
    pub unstake_account: Box<Account<'info, UnstakeAccount>>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct EventUnstake {
    pub era: u64,
    pub staker: Pubkey,
    pub burn_lsd_token_from: Pubkey,
    pub unstake_account: Pubkey,
    pub unstake_amount: u64,
    pub sol_amount: u64,
}

impl<'info> Unstake<'info> {
    pub fn process(&mut self, unstake_amount: u64) -> Result<()> {
        require_gt!(unstake_amount, 0, Errors::UnstakeAmountIsZero);

        if self
            .burn_lsd_token_from
            .delegate
            .contains(self.burn_lsd_token_authority.key)
        {
            require_gte!(
                self.burn_lsd_token_from.delegated_amount,
                unstake_amount,
                Errors::BalanceNotEnough
            );
        } else if self.burn_lsd_token_authority.key() == self.burn_lsd_token_from.owner {
            require_gte!(
                self.burn_lsd_token_from.amount,
                unstake_amount,
                Errors::BalanceNotEnough
            );
        } else {
            return err!(Errors::AuthorityNotMatch);
        }

        let sol_amount = self.stake_manager.calc_sol_amount(unstake_amount)?;
        self.stake_manager.era_unbond += sol_amount;
        self.stake_manager.active -= sol_amount;

        // burn lsd token
        burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                Burn {
                    mint: self.lsd_token_mint.to_account_info(),
                    from: self.burn_lsd_token_from.to_account_info(),
                    authority: self.burn_lsd_token_authority.to_account_info(),
                },
            ),
            unstake_amount,
        )?;

        self.unstake_account.set_inner(UnstakeAccount {
            stake_manager: self.stake_manager.key(),
            recipient: self.burn_lsd_token_from.owner,
            amount: sol_amount,
            created_epoch: Clock::get().unwrap().epoch,
        });

        emit!(EventUnstake {
            era: self.stake_manager.latest_era,
            staker: self.burn_lsd_token_from.owner,
            burn_lsd_token_from: self.burn_lsd_token_from.key(),
            unstake_account: self.unstake_account.key(),
            unstake_amount,
            sol_amount
        });

        Ok(())
    }
}
