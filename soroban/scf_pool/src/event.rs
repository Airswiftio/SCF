use soroban_sdk::{symbol_short,Address, Env};


pub fn deposit(e:&Env, from:Address, amount: i128){
    let topics = (symbol_short!("deposit"), from);
    e.events().publish(topics, amount);
}

pub fn withdraw(e:&Env, to:Address, amount:i128){
    let topics = (symbol_short!("withdraw"), to);
    e.events().publish(topics, amount);
}

pub fn create_offer(e: &Env, from: Address, offer_id: i128, amount: i128) {
    let topics = (symbol_short!("create"), from, amount);
    e.events().publish(topics, offer_id.clone());
}

pub fn expire_offer(e: &Env, from: Address, offer_id: i128) {
    let topics = (symbol_short!("expire"), from);
    e.events().publish(topics, offer_id.clone());
}

pub fn accept_offer(e: &Env, to: Address, offer_id: i128) {
    let topics = (symbol_short!("accept"), to.clone());
    e.events().publish(topics, offer_id.clone());
}