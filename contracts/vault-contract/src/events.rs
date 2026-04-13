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

pub fn initialized(e: &Env, admin: Address, token: Address) {
    let topics = (Symbol::new(e, "initialized"), admin);
    e.events().publish(topics, token);
}

// TODO: Add events for governance changes
// TODO: Add more detailed event logging for auditability
