use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn deposit(e: &Env, user: Address, amount: i128) {
    let topics = (symbol_short!("deposit"), user);
    e.events().publish(topics, amount);
}

pub fn withdraw(e: &Env, user: Address, amount: i128) {
    let topics = (symbol_short!("withdraw"), user);
    e.events().publish(topics, amount);
}

pub fn reward_distributed(e: &Env, amount: i128) {
    let topics = (symbol_short!("reward"),);
    e.events().publish(topics, amount);
}

pub fn reward_claimed(e: &Env, user: Address, amount: i128) {
    let topics = (Symbol::new(e, "reward_claimed"), user);
    e.events().publish(topics, amount);
}

pub fn capacity_updated(e: &Env, new_capacity: i128) {
    let topics = (Symbol::new(e, "cap_updated"),);
    e.events().publish(topics, new_capacity);
}

pub fn initialized(e: &Env, admin: Address, token: Address) {
    let topics = (Symbol::new(e, "initialized"), admin);
    e.events().publish(topics, token);
}
