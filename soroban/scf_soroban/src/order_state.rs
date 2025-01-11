use crate::balance::read_supply;

use crate::order_info::read_order_info;
use crate::owner::read_owner;
use crate::remainder::{has_remainder, read_remainder};
use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use crate::sub_tc::{
    read_sub_tc, read_sub_tc_disabled, update_sub_tc_amount, write_sub_tc_disabled,
};
use soroban_sdk::Env;

pub fn update_and_read_expired(env: &Env) -> bool {
    let expired_cached = read_expired(&env);
    if expired_cached {
        return true;
    }
    let ledger = env.ledger();
    let expired = ledger.timestamp() >= read_order_info(&env).end_time;
    if expired {
        write_expired(&env, true);
        // find unclaimed TCs (TCs owned by the contract)
        let last_id = read_supply(&env);
        if last_id > 0 {
            let contract_addr = &env.current_contract_address();
            for i in 1..last_id {
                let sub_tc = read_sub_tc(&env, i);
                let owner = read_owner(&env, i);
                // Set unclaimed TCs to disabled, and add their value to the parent's associated remainder TC
                if owner == contract_addr.clone() && !read_sub_tc_disabled(&env, i) {
                    let mut target = sub_tc.parent;
                    while has_remainder(&env, target) {
                        target = read_remainder(&env, target);
                    }
                    let target_tc = read_sub_tc(&env, target);
                    update_sub_tc_amount(&env, target, target_tc.amount + sub_tc.amount);
                    write_sub_tc_disabled(&env, i, true);
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
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
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
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_paid(env: &Env) -> bool {
    let key = DataKey::Paid;
    match env.storage().persistent().get::<DataKey, bool>(&key) {
        Some(data) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
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
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
