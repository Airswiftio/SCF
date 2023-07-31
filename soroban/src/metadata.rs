use crate::{storage_types::DataKey,storage_types::NFTInfo};
use soroban_sdk::{Env, Symbol, String, panic_with_error};
use crate::errors::Error;

pub fn read_name(env: &Env) -> String {
    let key = DataKey::Name;
    match env.storage().persistent().get::<DataKey, String>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_name(env: &Env, name: &String) {
    let key = DataKey::Name;
    env.storage().persistent().set(&key, name)
}

pub fn read_symbol(env: &Env) -> Symbol {
    let key = DataKey::Symbol;
    match env.storage().persistent().get::<DataKey, Symbol>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_symbol(env: &Env, symbol: &Symbol) {
    let key = DataKey::Symbol;
    env.storage().persistent().set(&key, symbol)
}

pub fn read_token_uri(env: &Env, id: i128) -> String {
    let key = DataKey::URI(id);
    match env.storage().persistent().get::<DataKey, String>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_token_uri(env: &Env, id: i128, uri: String) {
    let key = DataKey::URI(id);
    env.storage().persistent().set(&key, &uri)
}

pub fn read_nft_info(env: &Env, id: i128) -> NFTInfo {
    let key = DataKey::NFTInfo(id);
    match env.storage().persistent().get::<DataKey, NFTInfo>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

pub fn write_nft_info(env: &Env, id: i128, root: i128, amount: u32) {
    let key = DataKey::NFTInfo(id);
    env.storage().persistent().set(&key, &NFTInfo{root,amount})
}