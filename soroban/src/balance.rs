use crate::{interface::WriteType, storage_types::DataKey};
use soroban_sdk::{Address, Env};

pub fn read_supply(env: &Env) -> i128 {
    let key = DataKey::Supply;
    match env.storage().persistent().get::<DataKey, i128>(&key) {
        Some(balance) => balance,
        None => 0,
    }
}

pub fn increment_supply(env: &Env) {
    let key = DataKey::Supply;
    env.storage().persistent().set(&key, &(read_supply(&env) + 1));
}

pub fn read_minted(env: &Env, owner: Address) -> bool {
    let key = DataKey::Minted(owner);
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(minted) => minted,
        None => false,
    }
}

pub fn write_minted(env: &Env, owner: Address) {
    let key = DataKey::Minted(owner);
    env.storage().persistent().set(&key, &true);
}

pub fn check_minted(env: &Env, owner: Address) {
    assert!(!read_minted(&env, owner), "already minted");
}