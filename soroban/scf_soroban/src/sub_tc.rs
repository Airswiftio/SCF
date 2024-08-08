use crate::errors::Error;
use crate::storage_types::{DataKey, SubTC, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{panic_with_error, Env};

pub fn read_sub_tc(env: &Env, id: i128) -> SubTC {
    let key = DataKey::SubTCInfo(id);
    match env.storage().persistent().get::<DataKey, SubTC>(&key) {
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

pub fn write_sub_tc(env: &Env, id: i128, parent: i128, depth: u32, amount: u32) {
    let key = DataKey::SubTCInfo(id);
    match env.storage().persistent().get::<DataKey, SubTC>(&key) {
        Some(_) => panic_with_error!(env, Error::NotEmpty),
        None => {
            let sub_tc = SubTC {
                parent,
                depth,
                amount,
            };
            env.storage().persistent().set(&key, &sub_tc);
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
    }
}

pub fn read_sub_tc_disabled(env: &Env, id: i128) -> bool {
    let key = DataKey::Disabled(id);
    match env.storage().persistent().get::<DataKey, bool>(&key) {
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

pub fn write_sub_tc_disabled(env: &Env, id: i128, disabled: bool) {
    let key = DataKey::Disabled(id);
    env.storage().persistent().set(&key, &disabled);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_loan_status(env: &Env, id: i128) -> u32 {
    let key = DataKey::LoanStatus(id);
    match env.storage().persistent().get::<DataKey, u32>(&key) {
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

pub fn write_loan_status(env: &Env, id: i128, status: u32) {
    let key = DataKey::LoanStatus(id);
    env.storage().persistent().set(&key, &status);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
