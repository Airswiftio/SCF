use soroban_sdk::{symbol_short, Address, Env, Vec};

pub(crate) fn set_admin(e: &Env, admin: Address, new_admin: Address) {
    let topics = (symbol_short!("set_admin"), admin);
    e.events().publish(topics, new_admin);
}

pub(crate) fn transfer(e: &Env, from: Address, to: Address, id: i128) {
    let topics = (symbol_short!("transfer"), from, to);
    e.events().publish(topics, id);
}

pub(crate) fn mint(e: &Env, to: Address, id: i128) {
    let topics = (symbol_short!("mint"), to);
    e.events().publish(topics, id);
}

pub(crate) fn burn(e: &Env, from: Address, id: i128) {
    let topics = (symbol_short!("burn"), from);
    e.events().publish(topics, id);
}

pub(crate) fn redeem(e: &Env, owner: Address, id: i128) {
    let topics = (symbol_short!("redeem"), owner);
    e.events().publish(topics, id);
}

pub(crate) fn split(e: &Env, from: Address, id: i128, new_ids: Vec<i128>) {
    let topics = (symbol_short!("split"), from);
    e.events().publish(topics, (id, new_ids));
}
