#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token};

mod errors;
mod events;
mod storage;

use crate::errors::VaultError;
use crate::storage::{
    get_admin, get_token, get_total_shares, get_user_balance, get_reward_per_share,
    get_user_reward_debt, get_vault_capacity, set_admin, set_token, set_total_shares,
    set_user_balance, set_reward_per_share, set_user_reward_debt, set_vault_capacity,
};

const PRECISION: i128 = 1_000_000_000;

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultContract {
    /// Initializes the vault with an admin and a token address.
    pub fn initialize(e: Env, admin: Address, token: Address) -> Result<(), VaultError> {
        if get_admin(&e).is_some() {
            return Err(VaultError::AlreadyInitialized);
        }
        set_admin(&e, &admin);
        set_token(&e, &token);
        set_total_shares(&e, 0);
        set_reward_per_share(&e, 0);

        events::initialized(&e, admin, token);
        Ok(())
    }

    /// Deposits tokens into the vault and updates reward debt.
    /// Fix #1: Correctly accumulates balance by reading existing balance before adding.
    /// Fix #13: Enforces vault capacity limit if set.
    pub fn deposit(e: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();

        if amount <= 0 {
            return Err(VaultError::NegativeAmount);
        }

        // Fix #13: Check vault capacity before accepting deposit
        let total_shares = get_total_shares(&e);
        if let Some(capacity) = get_vault_capacity(&e) {
            if total_shares + amount > capacity {
                return Err(VaultError::CapacityExceeded);
            }
        }

        let token_address = get_token(&e).ok_or(VaultError::NotInitialized)?;
        let token_client = token::Client::new(&e, &token_address);

        token_client.transfer(&user, &e.current_contract_address(), &amount);

        // Fix #1: Read existing balance and add to it (additive accumulation)
        let user_balance = get_user_balance(&e, &user);
        let new_balance = user_balance + amount;
        set_user_balance(&e, &user, new_balance);
        set_total_shares(&e, total_shares + amount);

        // Update reward debt based on new balance
        let reward_per_share = get_reward_per_share(&e);
        let new_debt = new_balance * reward_per_share / PRECISION;
        set_user_reward_debt(&e, &user, new_debt);

        events::deposit(&e, user, amount);
        Ok(())
    }

    /// Withdraws tokens from the vault.
    pub fn withdraw(e: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();

        if amount <= 0 {
            return Err(VaultError::NegativeAmount);
        }

        let user_balance = get_user_balance(&e, &user);
        if user_balance < amount {
            return Err(VaultError::InsufficientBalance);
        }

        let token_address = get_token(&e).ok_or(VaultError::NotInitialized)?;
        let token_client = token::Client::new(&e, &token_address);

        token_client.transfer(&e.current_contract_address(), &user, &amount);

        let total_shares = get_total_shares(&e);
        let new_balance = user_balance - amount;
        set_user_balance(&e, &user, new_balance);
        set_total_shares(&e, total_shares - amount);

        // Update reward debt
        let reward_per_share = get_reward_per_share(&e);
        let new_debt = new_balance * reward_per_share / PRECISION;
        set_user_reward_debt(&e, &user, new_debt);

        events::withdraw(&e, user, amount);
        Ok(())
    }

    /// Distributes rewards to all depositors by updating reward per share.
    pub fn distribute_rewards(e: Env, admin: Address, amount: i128) -> Result<(), VaultError> {
        admin.require_auth();
        let current_admin = get_admin(&e).ok_or(VaultError::NotInitialized)?;
        if admin != current_admin {
            return Err(VaultError::Unauthorized);
        }

        if amount <= 0 {
            return Err(VaultError::NegativeAmount);
        }

        let total_shares = get_total_shares(&e);
        if total_shares == 0 {
            return Ok(());
        }

        let token_address = get_token(&e).ok_or(VaultError::NotInitialized)?;
        let token_client = token::Client::new(&e, &token_address);
        token_client.transfer(&admin, &e.current_contract_address(), &amount);

        let current_reward_per_share = get_reward_per_share(&e);
        let reward_increase = (amount * PRECISION) / total_shares;
        set_reward_per_share(&e, current_reward_per_share + reward_increase);

        events::reward_distributed(&e, amount);
        Ok(())
    }

    /// Fix #4: Allows users to claim their accumulated rewards.
    pub fn claim_rewards(e: Env, user: Address) -> Result<i128, VaultError> {
        user.require_auth();

        let pending = Self::calculate_pending_reward(e.clone(), user.clone())?;
        if pending <= 0 {
            return Err(VaultError::NoRewardsToClaim);
        }

        let token_address = get_token(&e).ok_or(VaultError::NotInitialized)?;
        let token_client = token::Client::new(&e, &token_address);
        token_client.transfer(&e.current_contract_address(), &user, &pending);

        // Reset reward debt to current accumulated value
        let balance = get_user_balance(&e, &user);
        let reward_per_share = get_reward_per_share(&e);
        let new_debt = balance * reward_per_share / PRECISION;
        set_user_reward_debt(&e, &user, new_debt);

        events::reward_claimed(&e, user, pending);
        Ok(pending)
    }

    /// Fix #8: Calculates pending rewards with precision handling for small deposits.
    /// Uses scaled arithmetic to avoid rounding to zero for small balances.
    pub fn calculate_pending_reward(e: Env, user: Address) -> Result<i128, VaultError> {
        let balance = get_user_balance(&e, &user);
        let reward_per_share = get_reward_per_share(&e);
        let debt = get_user_reward_debt(&e, &user);

        // Fix #8: Compute accumulated reward using full precision before dividing
        // balance * reward_per_share may be large; divide last to preserve precision
        let accumulated_reward = (balance * reward_per_share) / PRECISION;
        let pending = accumulated_reward - debt;
        Ok(if pending < 0 { 0 } else { pending })
    }

    /// Fix #13: Admin function to set the vault capacity limit.
    pub fn set_capacity(e: Env, admin: Address, capacity: i128) -> Result<(), VaultError> {
        admin.require_auth();
        let current_admin = get_admin(&e).ok_or(VaultError::NotInitialized)?;
        if admin != current_admin {
            return Err(VaultError::Unauthorized);
        }

        if capacity <= 0 {
            return Err(VaultError::NegativeAmount);
        }

        set_vault_capacity(&e, capacity);
        events::capacity_updated(&e, capacity);
        Ok(())
    }

    /// Returns the balance of a user.
    pub fn get_balance(e: Env, user: Address) -> i128 {
        get_user_balance(&e, &user)
    }

    /// Returns the total shares in the vault.
    pub fn get_total_shares(e: Env) -> i128 {
        get_total_shares(&e)
    }
}

mod test;
