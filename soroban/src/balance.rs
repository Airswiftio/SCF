use crate::{interface::WriteType, storage_types::DataKey};
use soroban_sdk::{Address, Env};

pub fn read_balance(env: &Env, owner: Address) -> i128 {
    let key = DataKey::Balance(owner);
    match env.storage().get(&key) {
        Some(balance) => balance.unwrap(),
        None => 0,
    }
}

pub fn write_balance(env: &Env, owner: Address, write_type: WriteType) {
    let key = DataKey::Balance(owner.clone());
    let balance = read_balance(env, owner);

    match write_type {
        WriteType::Add => env.storage().set(&key, &(balance + 1)),
        WriteType::Remove => env.storage().set(&key, &(balance - 1)),
    }
}

pub fn read_supply(env: &Env) -> i128 {
    let key = DataKey::Supply;
    match env.storage().get(&key) {
        Some(balance) => balance.unwrap(),
        None => 0,
    }
}

pub fn increment_supply(env: &Env) {
    let key = DataKey::Supply;
    env.storage().set(&key, &(read_supply(&env) + 1));
}

pub fn read_minted(env: &Env, owner: Address) -> bool {
    let key = DataKey::Minted(owner);
    match env.storage().get(&key) {
        Some(minted) => minted.unwrap(),
        None => false,
    }
}

pub fn write_minted(env: &Env, owner: Address) {
    let key = DataKey::Minted(owner);
    env.storage().set(&key, &true);
}

pub fn check_minted(env: &Env, owner: Address) {
    assert!(!read_minted(&env, owner), "already minted");
}