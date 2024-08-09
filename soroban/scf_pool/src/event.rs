use soroban_sdk::{symbol_short, Address, Env};

pub(crate) fn set_admin(e: &Env, admin: Address, new_admin: Address) {
    let topics = (symbol_short!("set_admin"), admin);
    e.events().publish(topics, new_admin);
}

pub fn create_offer(e: &Env, from: Address, offer_id: i128, amount: i128, fee: i128) {
    let topics = (symbol_short!("create"), from, amount, fee);
    e.events().publish(topics, offer_id.clone());
}

pub fn expire_offer(e: &Env, from: Address, offer_id: i128) {
    let topics = (symbol_short!("expire"), from);
    e.events().publish(topics, offer_id.clone());
}

pub fn close_offer(e: &Env, from: Address, offer_id: i128, amount: i128) {
    let topics = (symbol_short!("close"), from, offer_id.clone());
    e.events().publish(topics, amount);
}

pub fn accept_offer(e: &Env, to: Address, offer_id: i128) {
    let topics = (symbol_short!("accept"), to.clone());
    e.events().publish(topics, offer_id.clone());
}
