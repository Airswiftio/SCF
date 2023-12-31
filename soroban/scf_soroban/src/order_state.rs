use crate::balance::read_supply;
use crate::event;

use crate::order_info::{read_end_time};
use crate::owner::{read_owner, write_owner};
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Env};

pub fn update_and_read_expired(env: &Env) -> bool {
    let expired_cached = read_expired(&env);
    if expired_cached {
        return true;
    }
    let ledger = env.ledger();
    let expired = ledger.timestamp() >= read_end_time(&env);
    if expired {
        write_expired(&env, true);
        // transfer unclaimed NFTs to the root NFT's owner address
        let last_id = read_supply(&env);
        if last_id > 0 {
            let to = read_owner(&env, 0);
            let contract_addr = &env.current_contract_address();
            for i in 1..last_id {
                let owner = read_owner(&env, i);
                if owner == contract_addr.clone() {
                    write_owner(&env, i, Some(to.clone()));
                    event::transfer(&env, contract_addr.clone(), to.clone(), i);
                }
            }
        }
    }
    expired
}

fn read_expired(env: &Env) -> bool {
    let key = DataKey::Expired;
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(data) => {
            env.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            data
        }
        None => false,
    }
}

fn write_expired(env: &Env, val: bool) {
    let key = DataKey::Expired;
    env.storage().persistent().set(&key, &val);
    env.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_paid(env: &Env) -> bool {
    let key = DataKey::Paid;
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(data) => {
            env.storage()
                .persistent()
                .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
            data
        }
        None => false,
    }
}

pub fn write_paid(env: &Env, val: bool) {
    let key = DataKey::Paid;
    env.storage().persistent().set(&key, &val);
    env.storage()
        .persistent()
        .bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
