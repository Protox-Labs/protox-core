#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token};

mod errors;
mod events;
mod storage;

use crate::errors::VaultError;
use crate::storage::{
    get_admin, get_token, get_total_shares, get_user_balance, get_reward_per_share,
    get_user_reward_debt, set_admin, set_token, set_total_shares, set_user_balance,
    set_reward_per_share, set_user_reward_debt,
    is_paused, set_paused,
    set_deposit_timestamp, cooldown_met,
};

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
    pub fn deposit(e: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();

         if amount <= 0 {
        return Err(VaultError::ZeroDepositAmount);
    }


        let token_address = get_token(&e).ok_or(VaultError::NotInitialized)?;
        let token_client = token::Client::new(&e, &token_address);

        // Calculate pending rewards before updating shares
        let pending_reward = Self::calculate_pending_reward(e.clone(), user.clone())?;
        if pending_reward > 0 {
            // In a real scenario, we might want to automatically distribute rewards here
            // For now, we update the user's reward debt
        }

        token_client.transfer(&user, &e.current_contract_address(), &amount);

        let user_balance = get_user_balance(&e, &user);
        let total_shares = get_total_shares(&e);

        set_user_balance(&e, &user, user_balance + amount);
        set_total_shares(&e, total_shares + amount);

        // Update reward debt
        let reward_per_share = get_reward_per_share(&e);
        let new_debt = (user_balance + amount) * reward_per_share / 1_000_000_000;
        set_user_reward_debt(&e, &user, new_debt);
        set_deposit_timestamp(&e, &user);

        events::deposit(&e, user, amount);
        Ok(())
    }

    /// Withdraws tokens from the vault.
    pub fn withdraw(e: Env, user: Address, amount: i128) -> Result<(), VaultError> {
    user.require_auth();

    if !cooldown_met(&e, &user) {
    return Err(VaultError::CooldownNotMet);
}

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
    set_user_balance(&e, &user, user_balance - amount);
    set_total_shares(&e, total_shares - amount);

    let reward_per_share = get_reward_per_share(&e);
    let new_debt = (user_balance - amount) * reward_per_share / 1_000_000_000;
    set_user_reward_debt(&e, &user, new_debt);

    events::withdraw(&e, user, amount);
    Ok(())
}

    // deduct balance
    env.storage()
        .persistent()
        .set(&user, &(balance - amount));let balance: i128 = env
        .storage()
        .persistent()
        .get(&user)
        .unwrap_or(0);

    if amount > balance {
        return Err(VaultError::InsufficientBalance);
    }

    // deduct balance
    env.storage()
        .persistent()
        .set(&user, &(balance - amount));

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
            return Ok(()); // No shares, no rewards to distribute
        }

        let token_address = get_token(&e).ok_or(VaultError::NotInitialized)?;
        let token_client = token::Client::new(&e, &token_address);
        token_client.transfer(&admin, &e.current_contract_address(), &amount);

        let current_reward_per_share = get_reward_per_share(&e);
        // Using a scale factor for precision
        let reward_increase = (amount * 1_000_000_000) / total_shares;
        set_reward_per_share(&e, current_reward_per_share + reward_increase);

        events::reward_distributed(&e, amount);
        Ok(())
    }

    /// Calculates pending rewards for a user.
    pub fn calculate_pending_reward(e: Env, user: Address) -> Result<i128, VaultError> {
        let balance = get_user_balance(&e, &user);
        let reward_per_share = get_reward_per_share(&e);
        let debt = get_user_reward_debt(&e, &user);

        let accumulated_reward = (balance * reward_per_share) / 1_000_000_000;
        Ok(accumulated_reward - debt)
    }

    /// Returns the balance of a user.
    pub fn get_balance(e: Env, user: Address) -> i128 {
        get_user_balance(&e, &user)
    }

    /// Returns the total shares in the vault.
    pub fn get_total_shares(e: Env) -> i128 {
        get_total_shares(&e)
    }

    #[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    InsufficientBalance = 101, // from issue #3
    ZeroDepositAmount   = 102, // new
}

pub enum VaultError {
    // ...existing errors...
    InsufficientBalance = 101, // pick the next available number
}

    // TODO: Implement reward claiming logic
    // TODO: Add support for multiple tokens for rewards
    // TODO: Implement gas optimization for reward calculations
    // TODO: Add security checks for reward distribution frequency
    // TODO: Implement governance mechanism for admin updates
}

mod test;
