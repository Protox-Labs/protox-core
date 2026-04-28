#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, token};

fn setup() -> (Env, Address, Address, Address, Address, VaultContractClient<'static>) {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    token_admin_client.mint(&user, &10_000);
    token_admin_client.mint(&admin, &10_000);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);
    client.initialize(&admin, &token_id);
    // Return owned values; caller reconstructs clients as needed
    (e, admin, user, token_id, contract_id, client)
}

#[test]
fn test_initialize() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let token = Address::generate(&e);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token);
    assert_eq!(client.get_total_shares(), 0);
}

#[test]
fn test_deposit() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&e, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);

    token_admin_client.mint(&user, &1000);
    client.deposit(&user, &500);

    assert_eq!(client.get_balance(&user), 500);
    assert_eq!(client.get_total_shares(), 500);
    assert_eq!(token_client.balance(&user), 500);
    assert_eq!(token_client.balance(&contract_id), 500);
}

/// Issue #1: Multiple deposits must accumulate (not overwrite) the balance.
#[test]
fn test_multiple_deposits_accumulate_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);
    token_admin_client.mint(&user, &1000);

    client.deposit(&user, &300);
    assert_eq!(client.get_balance(&user), 300);

    client.deposit(&user, &200);
    assert_eq!(client.get_balance(&user), 500); // must be 300 + 200, not 200

    client.deposit(&user, &100);
    assert_eq!(client.get_balance(&user), 600); // must be 500 + 100, not 100

    assert_eq!(client.get_total_shares(), 600);
}

#[test]
fn test_withdraw() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&e, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);

    token_admin_client.mint(&user, &1000);
    client.deposit(&user, &500);
    client.withdraw(&user, &200);

    assert_eq!(client.get_balance(&user), 300);
    assert_eq!(client.get_total_shares(), 300);
    assert_eq!(token_client.balance(&user), 700);
    assert_eq!(token_client.balance(&contract_id), 300);
}

#[test]
fn test_reward_distribution() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);

    token_admin_client.mint(&user1, &1000);
    token_admin_client.mint(&user2, &1000);
    token_admin_client.mint(&admin, &1000);

    client.deposit(&user1, &500);
    client.deposit(&user2, &500);

    client.distribute_rewards(&admin, &100);

    let reward1 = client.calculate_pending_reward(&user1);
    let reward2 = client.calculate_pending_reward(&user2);

    assert_eq!(reward1, 50);
    assert_eq!(reward2, 50);
}

/// Issue #4: Users can claim accumulated rewards.
#[test]
fn test_claim_rewards() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&e, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);
    token_admin_client.mint(&user, &1000);
    token_admin_client.mint(&admin, &500);

    client.deposit(&user, &1000);
    client.distribute_rewards(&admin, &100);

    let pending = client.calculate_pending_reward(&user);
    assert_eq!(pending, 100);

    let balance_before = token_client.balance(&user);
    let claimed = client.claim_rewards(&user);
    assert_eq!(claimed, 100);
    assert_eq!(token_client.balance(&user), balance_before + 100);

    // After claiming, pending rewards should be 0
    assert_eq!(client.calculate_pending_reward(&user), 0);
}

/// Issue #8: Small deposits should still receive proportional rewards (no rounding to zero).
#[test]
fn test_small_deposit_reward_precision() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);
    token_admin_client.mint(&user, &10);
    token_admin_client.mint(&admin, &1_000_000);

    // Small deposit: 1 token
    client.deposit(&user, &1);

    // Large reward relative to deposit: 1_000_000 tokens distributed to 1 share
    client.distribute_rewards(&admin, &1_000_000);

    let pending = client.calculate_pending_reward(&user);
    // user has 1 share out of 1 total, so should receive all 1_000_000
    assert_eq!(pending, 1_000_000);
}

/// Issue #13: Vault capacity limit is enforced.
#[test]
fn test_vault_capacity_limit() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);
    token_admin_client.mint(&user, &2000);

    // Set capacity to 500
    client.set_capacity(&admin, &500);

    // Deposit within capacity succeeds
    client.deposit(&user, &500);
    assert_eq!(client.get_total_shares(), 500);

    // Deposit exceeding capacity fails
    let result = client.try_deposit(&user, &1);
    assert!(result.is_err());
}

/// Issue #13: Admin can update the capacity.
#[test]
fn test_capacity_can_be_updated() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token_admin = Address::generate(&e);
    let token_id = e.register_stellar_asset_contract(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);
    token_admin_client.mint(&user, &2000);

    client.set_capacity(&admin, &500);
    client.deposit(&user, &500);

    // Increase capacity and deposit more
    client.set_capacity(&admin, &1000);
    client.deposit(&user, &500);
    assert_eq!(client.get_total_shares(), 1000);
}
