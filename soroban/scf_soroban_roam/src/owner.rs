use crate::balance::read_supply;
use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use crate::sub_tc::read_sub_tc_disabled;
use soroban_sdk::{panic_with_error, vec, Address, Env, String, Vec};

pub fn read_owner(env: &Env, id: i128) -> Address {
    let key = DataKey::Owner(id);
    match env.storage().persistent().get::<DataKey, Address>(&key) {
        Some(balance) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            balance
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_owner(env: &Env, id: i128, owner: Option<Address>) {
    let key = DataKey::Owner(id);
    match owner {
        Some(owner) => {
            env.storage().persistent().set(&key, &owner);
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
        None => env.storage().persistent().remove(&key),
    }
}

pub fn check_owner(env: &Env, auth: &Address, id: i128) {
    if auth != &read_owner(env, id) {
        panic_with_error!(env, Error::NotOwned)
    }
}

pub fn read_recipient(env: &Env, id: i128) -> Address {
    let key = DataKey::Recipient(id);
    match env.storage().persistent().get::<DataKey, Address>(&key) {
        Some(data) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            data
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_recipient(env: &Env, id: i128, recipient: &Address) {
    let key = DataKey::Recipient(id);
    env.storage().persistent().set(&key, recipient);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_all_owned(env: &Env, address: Address) -> Vec<i128> {
    let mut ids = Vec::new(&env);
    let supply = read_supply(&env);
    if supply > 0 {
        for n in 0..supply {
            let owner = read_owner(&env, n);
            if owner == address && !read_sub_tc_disabled(&env, n) {
                ids.push_back(n);
            }
        }
    }
    ids
}

pub fn write_vc(env: &Env, id: i128, vc: Vec<String>) {
    let key = DataKey::VC(id);
    env.storage().persistent().set(&key, &vc);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn add_vc(env: &Env, id: i128, vc: String) {
    if vc.len() > 2048 {
        panic_with_error!(env, Error::VCStringTooLong);
    }
    let key = DataKey::VC(id);
    match env.storage().persistent().get::<DataKey, Vec<String>>(&key) {
        Some(mut vcs) => {
            if vcs.len() >= 10 {
                panic_with_error!(env, Error::VCListCapacityReached);
            }
            vcs.push_back(vc);
            env.storage().persistent().set(&key, &vcs);
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
        None => {
            env.storage().persistent().set(&key, &vec![&env, vc]);
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
    }
}

pub fn read_vc(env: &Env, id: i128) -> Vec<String> {
    let key = DataKey::VC(id);
    match env.storage().persistent().get::<DataKey, Vec<String>>(&key) {
        Some(data) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            data
        }
        None => panic_with_error!(env, Error::NotFound),
    }
}
