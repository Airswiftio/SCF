use crate::{
    errors::Error,
    storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{panic_with_error, BytesN, Env, Vec};

pub fn write_amount(e: &Env, id: u64, amount: u64) {
    let key = DataKey::Amount(id);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_amount(e: &Env, id: u64) -> u64 {
    let key = DataKey::Amount(id);
    match e.storage().persistent().get(&key) {
        Some(amount) => {
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            amount
        }
        None => 0,
    }
}

pub fn write_file_hashes(e: &Env, id: u64, metadata: Vec<BytesN<32>>) {
    let key = DataKey::FileHashes(id);
    e.storage().persistent().set(&key, &metadata);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_file_hashes(e: &Env, id: u64) -> Vec<BytesN<32>> {
    let key = DataKey::FileHashes(id);
    match e.storage().persistent().get(&key) {
        Some(data) => {
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            data
        }
        None => panic_with_error!(&e, Error::NotFound),
    }
}

pub fn write_redeem_time(e: &Env, id: u64, redeem_time: u64) {
    let key = DataKey::RedeemTime(id);
    e.storage().persistent().set(&key, &redeem_time);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_redeem_time(e: &Env, id: u64) -> u64 {
    let key = DataKey::RedeemTime(id);
    match e.storage().persistent().get(&key) {
        Some(redeem_time) => {
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            redeem_time
        }
        None => panic_with_error!(&e, Error::NotFound),
    }
}
