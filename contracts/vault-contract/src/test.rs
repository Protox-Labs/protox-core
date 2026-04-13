#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env, token};

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
    let token_client = token::Client::new(&e, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&e, &token_id);
    let contract_id = e.register_contract(None, VaultContract);
    let client = VaultContractClient::new(&e, &contract_id);

    client.initialize(&admin, &token_id);

    token_admin_client.mint(&user1, &1000);
    token_admin_client.mint(&user2, &1000);
    token_admin_client.mint(&admin, &1000);

    client.deposit(&user1, &500);
    client.deposit(&user2, &500);

    // Distribute 100 tokens as reward
    client.distribute_rewards(&admin, &100);

    // Both user1 and user2 should have 50 tokens as pending reward
    let reward1 = client.calculate_pending_reward(&user1);
    let reward2 = client.calculate_pending_reward(&user2);

    assert_eq!(reward1, 50);
    assert_eq!(reward2, 50);
}
