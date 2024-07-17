use std::collections::VecDeque;

pub use crate::errors::Errors;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct Stack {
    pub admin: Pubkey,
    pub stack_fee_commission: u64, // decimals 9
    pub stake_managers_len_limit: u64,
    pub entrusted_stake_managers: Vec<Pubkey>,
}

impl Stack {
    pub const DEFAULT_STACK_FEE_COMMISSION: u64 = 100_000_000;
    pub const DEFAULT_STAKE_MANAGERS_LEN_LIMIT: u64 = 20;

    pub fn calc_stack_fee(&self, platform_fee_raw: u64) -> Result<u64> {
        u64::try_from(
            (platform_fee_raw as u128) * (self.stack_fee_commission as u128) / (1e9 as u128),
        )
        .map_err(|_| error!(Errors::CalculationFail))
    }
}

#[account]
#[derive(Debug)]
pub struct StakeManager {
    pub admin: Pubkey,
    pub balancer: Pubkey,
    pub stack: Pubkey,
    pub lsd_token_mint: Pubkey,
    pub pool_seed_bump: u8,
    pub rent_exempt_for_pool_acc: u64,

    pub min_stake_amount: u64,
    pub platform_fee_commission: u64, // decimals 9
    pub stack_fee_commission: u64,
    pub rate_change_limit: u64, // decimals 9
    pub stake_accounts_len_limit: u64,
    pub split_accounts_len_limit: u64,
    pub unbonding_duration: u64,

    pub latest_era: u64,
    pub rate: u64, // decimals 9
    pub era_bond: u64,
    pub era_unbond: u64,
    pub active: u64,
    pub total_platform_fee: u64,
    pub validators: Vec<Pubkey>,
    pub stake_accounts: Vec<Pubkey>,
    pub split_accounts: Vec<Pubkey>,
    pub era_rates: VecDeque<EraRate>,
    pub era_process_data: EraProcessData,
}

#[derive(Clone, Debug, Default, AnchorSerialize, AnchorDeserialize)]
pub struct EraProcessData {
    pub need_bond: u64,
    pub need_unbond: u64,
    pub old_active: u64,
    pub new_active: u64,
    pub pending_stake_accounts: Vec<Pubkey>,
}

#[derive(Clone, Debug, Default, AnchorSerialize, AnchorDeserialize)]
pub struct EraRate {
    pub era: u64,
    pub rate: u64,
}

impl EraProcessData {
    pub fn is_empty(&self) -> bool {
        return self.need_bond == 0
            && self.need_unbond == 0
            && self.old_active == 0
            && self.new_active == 0
            && self.pending_stake_accounts.is_empty();
    }

    pub fn need_bond(&self) -> bool {
        return self.need_bond > 0;
    }

    pub fn need_unbond(&self) -> bool {
        return self.need_unbond > 0;
    }

    pub fn need_update_active(&self) -> bool {
        return self.need_bond == 0
            || self.need_unbond == 0 && !self.pending_stake_accounts.is_empty();
    }

    pub fn need_update_rate(&self) -> bool {
        return self.need_bond == 0
            && self.need_unbond == 0
            && self.pending_stake_accounts.is_empty()
            && self.old_active != 0
            && self.new_active != 0;
    }
}

impl StakeManager {
    pub const POOL_SEED: &'static [u8] = b"pool_seed";

    pub const DEFAULT_UNBONDING_DURATION: u64 = 2;
    pub const CAL_BASE: u64 = 1_000_000_000;
    pub const DEFAULT_RATE: u64 = 1_000_000_000;
    pub const DEFAULT_MIN_STAKE_AMOUNT: u64 = 1_000_000;
    pub const DEFAULT_PLATFORM_FEE_COMMISSION: u64 = 100_000_000;
    pub const DEFAULT_RATE_CHANGE_LIMIT: u64 = 500_000;
    pub const DEFAULT_STAKE_ACCOUNT_LEN_LIMIT: u64 = 100;
    pub const DEFAULT_SPLIT_ACCOUNT_LEN_LIMIT: u64 = 20;
    pub const ERA_RATES_LEN_LIMIT: u64 = 10;

    pub fn calc_lsd_token_amount(&self, sol_amount: u64) -> Result<u64> {
        u64::try_from((sol_amount as u128) * (StakeManager::CAL_BASE as u128) / (self.rate as u128))
            .map_err(|_| error!(Errors::CalculationFail))
    }

    pub fn calc_sol_amount(&self, lsd_token_amount: u64) -> Result<u64> {
        u64::try_from(
            (lsd_token_amount as u128) * (self.rate as u128) / (StakeManager::CAL_BASE as u128),
        )
        .map_err(|_| error!(Errors::CalculationFail))
    }

    pub fn calc_platform_fee(&self, reward_sol: u64) -> Result<u64> {
        u64::try_from(
            (reward_sol as u128) * (self.platform_fee_commission as u128) / (self.rate as u128),
        )
        .map_err(|_| error!(Errors::CalculationFail))
    }

    pub fn calc_rate(&self, sol_amount: u64, lsd_token_amount: u64) -> Result<u64> {
        if sol_amount == 0 || lsd_token_amount == 0 {
            return Ok(StakeManager::CAL_BASE);
        }

        u64::try_from(
            (sol_amount as u128) * (StakeManager::CAL_BASE as u128) / (lsd_token_amount as u128),
        )
        .map_err(|_| error!(Errors::CalculationFail))
    }

    pub fn calc_rate_change(&self, old_rate: u64, new_rate: u64) -> Result<u64> {
        if old_rate == 0 {
            return Ok(0);
        }
        let diff = if old_rate > new_rate {
            old_rate - new_rate
        } else {
            new_rate - old_rate
        };

        u64::try_from((diff as u128) * (StakeManager::CAL_BASE as u128) / (old_rate as u128))
            .map_err(|_| error!(Errors::CalculationFail))
    }
}

#[account]
#[derive(Debug)]
pub struct UnstakeAccount {
    pub stake_manager: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub created_epoch: u64,
}

#[account]
#[derive(Debug)]
pub struct StackFeeAccount {
    pub bump: u8,
    pub amount: u64,
}
