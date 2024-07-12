pub use crate::errors::Errors;
use crate::EraProcessData;
use crate::Stack;
use crate::StackFeeAccount;
pub use crate::StakeManager;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct InitializeStakeManager<'info> {
    #[account(
        zero,
        rent_exempt = enforce,
    )]
    pub stake_manager: Box<Account<'info, StakeManager>>,

    pub stack: Box<Account<'info, Stack>>,

    #[account(
        seeds = [
            &stake_manager.key().to_bytes(),
            StakeManager::POOL_SEED,
        ],
        bump,
    )]
    pub stake_pool: SystemAccount<'info>,

    #[account(
        associated_token::mint = lsd_token_mint,
        associated_token::authority = admin,
    )]
    pub platform_fee_recipient: Box<Account<'info, TokenAccount>>,

    #[account(
        associated_token::mint = lsd_token_mint,
        associated_token::authority = stack.admin,
    )]
    pub stack_fee_recipient: Box<Account<'info, TokenAccount>>,

    #[account(
        zero,
        rent_exempt = enforce
    )]
    pub stack_fee_account: Box<Account<'info, StackFeeAccount>>,

    pub lsd_token_mint: Box<Account<'info, Mint>>,

    pub admin: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeStakeManagerData {
    pub stack: Pubkey,
    pub lsd_token_mint: Pubkey,
    pub validator: Pubkey,
}

impl<'info> InitializeStakeManager<'info> {
    pub fn process(
        &mut self,
        initialize_data: InitializeStakeManagerData,
        pool_seed_bump: u8,
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
            balancer: self.admin.key(),
            stack: initialize_data.stack,
            lsd_token_mint: initialize_data.lsd_token_mint,
            rent_exempt_for_pool_acc,
            pool_seed_bump,
            min_stake_amount: StakeManager::DEFAULT_MIN_STAKE_AMOUNT,
            platform_fee_commission: StakeManager::DEFAULT_PLATFORM_FEE_COMMISSION,
            rate_change_limit: StakeManager::DEFAULT_RATE_CHANGE_LIMIT,
            stake_accounts_len_limit: StakeManager::DEFAULT_STAKE_ACCOUNT_LEN_LIMIT,
            split_accounts_len_limit: StakeManager::DEFAULT_SPLIT_ACCOUNT_LEN_LIMIT,
            unbonding_duration: StakeManager::DEFAULT_UNBONDING_DURATION,
            latest_era: self.clock.epoch,
            rate: StakeManager::DEFAULT_RATE,
            total_platform_fee: 0,
            era_bond: 0,
            era_unbond: 0,
            active: 0,
            validators: vec![initialize_data.validator],
            stake_accounts: vec![],
            split_accounts: vec![],
            era_process_data: EraProcessData {
                need_bond: 0,
                need_unbond: 0,
                old_active: 0,
                new_active: 0,
                pending_stake_accounts: vec![],
            },
        });

        Ok(())
    }
}
