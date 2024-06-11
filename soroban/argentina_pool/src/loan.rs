use crate::{
    errors::Error,
    storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{contracttype, panic_with_error, Address, Env, Map};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum LoanStatus {
    Pending = 0,
    Active = 1,
    Paid = 2,
    Closed = 3,
}

#[derive(Clone)]
#[contracttype]
pub struct Loan {
    pub borrower: Address,
    pub creditor: Address,
    pub amount: i128,
    pub tc_address: Address,
    pub tc_id: u64,
    pub fee_percent: u32,
    pub status: LoanStatus,
}

pub fn write_fee_percent(e: &Env, fee_percent: u32) {
    let key = DataKey::FeePercent;
    e.storage().instance().set(&key, &fee_percent);
}

pub fn read_fee_percent(e: &Env) -> u32 {
    let key = DataKey::FeePercent;
    match e.storage().instance().get::<DataKey, u32>(&key) {
        Some(fee_percent) => fee_percent,
        None => 0,
    }
}

pub fn write_loan(e: &Env, offer_id: u64, loan: Loan) {
    let key = DataKey::Loan(offer_id);
    e.storage().persistent().set(&key, &loan);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_loan(e: &Env, offer_id: u64) -> Loan {
    let key = DataKey::Loan(offer_id);
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

pub fn read_supply(e: &Env) -> u64 {
    let key = DataKey::Supply;
    match e.storage().instance().get::<DataKey, u64>(&key) {
        Some(balance) => balance,
        None => 0,
    }
}

pub fn increment_supply(e: &Env) {
    let key = DataKey::Supply;
    e.storage().instance().set(&key, &(read_supply(&e) + 1));
}

pub fn read_whitelist(e: &Env) -> Map<Address, ()> {
    let key = DataKey::TCWhiteList;
    match e
        .storage()
        .instance()
        .get::<DataKey, Map<Address, ()>>(&key)
    {
        Some(whitelist) => whitelist,
        None => Map::new(&e),
    }
}

pub fn write_whitelist(e: &Env, whitelist: Map<Address, ()>) {
    let key = DataKey::TCWhiteList;
    e.storage().instance().set(&key, &whitelist);
}

pub fn is_whitelisted(e: &Env, tc_addr: Address) -> bool {
    read_whitelist(&e).contains_key(tc_addr)
}
