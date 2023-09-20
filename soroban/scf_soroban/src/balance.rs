use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn read_supply(env: &Env) -> i128 {
    let key = DataKey::Supply;
    match env.storage().persistent().get::<DataKey, i128>(&key) {
        Some(balance) => {
            env.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            balance
        }
        None => 0,
    }
}

pub fn increment_supply(env: &Env) {
    let key = DataKey::Supply;
    env.storage()
        .persistent()
        .set(&key, &(read_supply(&env) + 1));
    env.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_minted(env: &Env, owner: Address) -> bool {
    let key = DataKey::Minted(owner);
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(minted) => {
            env.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            minted
        }
        None => false,
    }
}

pub fn write_minted(env: &Env, owner: Address) {
    let key = DataKey::Minted(owner);
    env.storage().persistent().set(&key, &true);
    env.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn check_minted(env: &Env, owner: Address) {
    assert!(!read_minted(&env, owner), "already minted");
}
