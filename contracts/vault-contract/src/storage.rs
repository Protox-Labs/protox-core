use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Balance(Address),
    TotalShares,
    RewardPerShare,
    UserRewardDebt(Address),
    VaultCapacity,
}

pub fn get_admin(e: &Env) -> Option<Address> {
    e.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_token(e: &Env) -> Option<Address> {
    e.storage().instance().get(&DataKey::Token)
}

pub fn set_token(e: &Env, token: &Address) {
    e.storage().instance().set(&DataKey::Token, token);
}

pub fn get_user_balance(e: &Env, user: &Address) -> i128 {
    e.storage()
        .persistent()
        .get(&DataKey::Balance(user.clone()))
        .unwrap_or(0)
}

pub fn set_user_balance(e: &Env, user: &Address, amount: i128) {
    e.storage()
        .persistent()
        .set(&DataKey::Balance(user.clone()), &amount);
}

pub fn get_total_shares(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&DataKey::TotalShares)
        .unwrap_or(0)
}

pub fn set_total_shares(e: &Env, shares: i128) {
    e.storage().instance().set(&DataKey::TotalShares, &shares);
}

pub fn get_reward_per_share(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&DataKey::RewardPerShare)
        .unwrap_or(0)
}

pub fn set_reward_per_share(e: &Env, reward: i128) {
    e.storage().instance().set(&DataKey::RewardPerShare, &reward);
}

pub fn get_user_reward_debt(e: &Env, user: &Address) -> i128 {
    e.storage()
        .persistent()
        .get(&DataKey::UserRewardDebt(user.clone()))
        .unwrap_or(0)
}

pub fn set_user_reward_debt(e: &Env, user: &Address, debt: i128) {
    e.storage()
        .persistent()
        .set(&DataKey::UserRewardDebt(user.clone()), &debt);
}

pub fn get_vault_capacity(e: &Env) -> Option<i128> {
    e.storage().instance().get(&DataKey::VaultCapacity)
}

pub fn set_vault_capacity(e: &Env, capacity: i128) {
    e.storage().instance().set(&DataKey::VaultCapacity, &capacity);
}
