use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_external_token_provider(env: &Env) -> Address {
    let key = DataKey::ExternalTokenProvider;
    match env.storage().persistent().get::<DataKey, Address>(&key) {
        Some(data) => {
            env.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            data
        }
        None => panic_with_error!(env, Error::InvalidContract),
    }
}

pub fn write_external_token_provider(env: &Env, addr: Address) {
    let key = DataKey::ExternalTokenProvider;
    env.storage().persistent().set(&key, &addr);
    env.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
