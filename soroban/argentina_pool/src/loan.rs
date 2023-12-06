use crate::storage_types::DataKey;
use soroban_sdk::Env;

pub fn write_rate_percent(e: &Env, rate_percent: u32) {
    let key = DataKey::RatePercent;
    e.storage().instance().set(&key, &rate_percent);
}
