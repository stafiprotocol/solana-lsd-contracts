pub use crate::errors::Errors;
use crate::helper;
use crate::EraProcessData;
use crate::StackFeeAccount;
pub use crate::StakeManager;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use stack::Stack;

#[derive(Accounts)]
#[instruction(stake_manager_index: u8)]
pub struct InitializeStakeManager<'info> {
    #[account(
        init,
        space = 16384,
        payer = rent_payer,
        rent_exempt = enforce,
        seeds = [
            helper::STAKE_MANAGER_SEED,
            &admin.key().to_bytes(),
            &[stake_manager_index],
        ],
        bump,
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub stack: Box<Account<'info, Stack>>,

    #[account(
        rent_exempt = enforce,
        seeds = [
            &stake_manager.key().to_bytes(),
            helper::STAKE_POOL_SEED,
        ],
        bump,
    )]
    pub stake_pool: SystemAccount<'info>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<StackFeeAccount>(),
        payer = rent_payer,
        rent_exempt = enforce,
        seeds = [
            &stack.key().to_bytes(),
            &lsd_token_mint.key().to_bytes(),
        ],
        bump,
    )]
    pub stack_fee_account: Box<Account<'info, StackFeeAccount>>,

    #[account(
        init,
        payer = rent_payer,
        mint::decimals = 9,
        mint::authority = stake_pool,
        mint::freeze_authority = stake_pool,
        seeds = [
            helper::TOKEN_MINT_SEED,
            &admin.key().to_bytes(),
            &[stake_manager_index],
        ],
        bump,
    )]
    pub lsd_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: todo
    pub validator: AccountInfo<'info>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub admin: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeStakeManager<'info> {
    pub fn process(
        &mut self,
        _stake_manager_index: u8,
        pool_seed_bump: u8,
        stack_fee_account_seed_bump: u8,
    ) -> Result<()> {
        require_keys_neq!(self.stake_manager.key(), self.stake_pool.key());

        let rent_exempt_for_pool_acc = self.rent.minimum_balance(0);
        require_eq!(
            self.stake_pool.lamports(),
            rent_exempt_for_pool_acc,
            Errors::RentNotEnough
        );

        require!(
            self.lsd_token_mint
                .mint_authority
                .contains(&self.stake_pool.key()),
            Errors::MintAuthorityNotMatch
        );
        require!(
            self.lsd_token_mint.freeze_authority.is_none(),
            Errors::FreezeAuthorityNotMatch
        );
        require!(self.lsd_token_mint.supply == 0, Errors::MintSupplyNotEmpty);

        self.stake_manager.set_inner(StakeManager {
            admin: self.admin.key(),
            pending_admin: Pubkey::default(),
            balancer: self.admin.key(),
            stack: self.stack.key(),
            lsd_token_mint: self.lsd_token_mint.key(),
            rent_exempt_for_pool_acc,
            pool_seed_bump,
            min_stake_amount: StakeManager::DEFAULT_MIN_STAKE_AMOUNT,
            platform_fee_commission: StakeManager::DEFAULT_PLATFORM_FEE_COMMISSION,
            stack_fee_commission: self.stack.stack_fee_commission,
            rate_change_limit: StakeManager::DEFAULT_RATE_CHANGE_LIMIT,
            stake_accounts_len_limit: StakeManager::DEFAULT_STAKE_ACCOUNT_LEN_LIMIT,
            split_accounts_len_limit: StakeManager::DEFAULT_SPLIT_ACCOUNT_LEN_LIMIT,
            unbonding_duration: StakeManager::DEFAULT_UNBONDING_DURATION,
            latest_era: Clock::get().unwrap().epoch,
            rate: StakeManager::DEFAULT_RATE,
            total_platform_fee: 0,
            era_bond: 0,
            era_unbond: 0,
            active: 0,
            validators: vec![self.validator.key()],
            stake_accounts: vec![],
            split_accounts: vec![],
            era_rates: vec![],
            era_process_data: EraProcessData {
                need_bond: 0,
                need_unbond: 0,
                old_active: 0,
                new_active: 0,
                pending_stake_accounts: vec![],
            },
        });

        self.stack_fee_account.set_inner(StackFeeAccount {
            bump: stack_fee_account_seed_bump,
            amount: 0,
        });

        Ok(())
    }
}
