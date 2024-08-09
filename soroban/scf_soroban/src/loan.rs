use crate::errors::Error;
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn has_loan_contract(env: &Env) -> bool {
    let key = DataKey::LoanContract;
    env.storage().instance().has(&key)
}

pub fn read_loan_contract(env: &Env) -> Address {
    let key = DataKey::LoanContract;
    match env.storage().instance().get::<DataKey, Address>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::InvalidContract),
    }
}

pub fn write_loan_contract(env: &Env, contract_addr: &Address) {
    let key = DataKey::LoanContract;
    env.storage().instance().set(&key, contract_addr);
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
