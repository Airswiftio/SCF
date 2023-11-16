use crate::{
    errors::Error,
    storage_types::{DataKey, HashMetadata, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{panic_with_error, Env};

pub fn write_amount(e: &Env, id: i128, amount: u32) {
    let key = DataKey::Amount(id);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_amount(e: &Env, id: i128) -> u32 {
    let key = DataKey::Amount(id);
    match e.storage().persistent().get(&key) {
        Some(amount) => {
            e.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            amount
        }
        None => 0,
    }
}

pub fn write_metadata(e: &Env, id: i128, metadata: HashMetadata) {
    let key = DataKey::Metadata(id);
    e.storage().persistent().set(&key, &metadata);
    e.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_metadata(e: &Env, id: i128) -> HashMetadata {
    let key = DataKey::Metadata(id);
    match e.storage().persistent().get(&key) {
        Some(data) => {
            e.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            data
        }
        None => panic_with_error!(&e, Error::NotFound),
    }
}
