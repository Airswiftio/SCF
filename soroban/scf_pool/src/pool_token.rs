use soroban_sdk::{Address, Env, Map};

use crate::storage_types::DataKey;

pub fn read_ext_tokens(e: &Env) -> Map<Address, ()> {
    let key = DataKey::ExtTokens;
    match e
        .storage()
        .instance()
        .get::<DataKey, Map<Address, ()>>(&key)
    {
        Some(whitelist) => whitelist,
        None => Map::new(&e),
    }
}

pub fn write_ext_tokens(e: &Env, whitelist: Map<Address, ()>) {
    let key = DataKey::ExtTokens;
    e.storage().instance().set(&key, &whitelist);
}

pub fn has_ext_token(e: &Env, ext_token_addr: Address) -> bool {
    read_ext_tokens(&e).contains_key(ext_token_addr)
}
