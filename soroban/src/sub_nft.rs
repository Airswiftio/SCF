use crate::{storage_types::DataKey,storage_types::SubNFT};
use soroban_sdk::{Env, Symbol, String, panic_with_error, Map, Address, Vec};
use crate::errors::Error;

pub fn read_sub_nft(env: &Env, id: i128) -> SubNFT {
    let key = DataKey::SubNFTInfo(id);
    match env.storage().persistent().get::<DataKey, SubNFT>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_sub_nft(env: &Env, id: i128, root: i128, amount: u32) {
    let key = DataKey::SubNFTInfo(id);
    match env.storage().persistent().get::<DataKey, SubNFT>(&key) {
        Some(_) => panic_with_error!(env, Error::NotEmpty),
        None => {
            let sub_nft = SubNFT{root, amount};
            env.storage().persistent().set(&key, &sub_nft);
        }
    }
}

pub fn read_sub_nft_disabled(env: &Env, id: i128) -> bool {
    let key = DataKey::Disabled(id);
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(data) => data,
        None => false,
    }
}

pub fn write_sub_nft_disabled(env: &Env, id: i128, disabled: bool) {
    let key = DataKey::Disabled(id);
    env.storage().persistent().set(&key, &disabled);
}