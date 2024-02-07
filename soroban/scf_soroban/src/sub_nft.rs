use crate::errors::Error;
use crate::storage_types::{DataKey, SubNFT, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{panic_with_error, Env, String};

pub fn read_sub_nft(env: &Env, id: i128) -> SubNFT {
    let key = DataKey::SubNFTInfo(id);
    match env.storage().persistent().get::<DataKey, SubNFT>(&key) {
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

pub fn write_sub_nft(env: &Env, id: i128, root: i128, amount: u32, data: String) {
    let key = DataKey::SubNFTInfo(id);
    match env.storage().persistent().get::<DataKey, SubNFT>(&key) {
        Some(_) => panic_with_error!(env, Error::NotEmpty),
        None => {
            let sub_nft = SubNFT { root, amount, data };
            env.storage().persistent().set(&key, &sub_nft);
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
    }
}

pub fn read_sub_nft_disabled(env: &Env, id: i128) -> bool {
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

pub fn write_sub_nft_disabled(env: &Env, id: i128, disabled: bool) {
    let key = DataKey::Disabled(id);
    env.storage().persistent().set(&key, &disabled);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn update_sub_nft_data(env: &Env, id: i128, data: String) {
    let mut sub_nft = read_sub_nft(env, id);
    sub_nft.data = data;
    let key = DataKey::SubNFTInfo(id);
    env.storage().persistent().set(&key, &sub_nft);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
