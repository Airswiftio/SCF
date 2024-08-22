use crate::{errors::Error, storage_types::DataKey};
use soroban_sdk::{contracttype, panic_with_error, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct TokenOrderInfo {
    pub buyer_address: Address,
    pub total_amount: u32,
    pub end_time: u64,
}

pub fn write_order_info(env: &Env, buyer_address: Address, total_amount: u32, end_time: u64) {
    let key = DataKey::OrderInfo;
    let order_info = TokenOrderInfo {
        buyer_address,
        total_amount,
        end_time,
    };
    env.storage().instance().set(&key, &order_info);
}

pub fn read_order_info(env: &Env) -> TokenOrderInfo {
    let key = DataKey::OrderInfo;
    match env
        .storage()
        .instance()
        .get::<DataKey, TokenOrderInfo>(&key)
    {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}
