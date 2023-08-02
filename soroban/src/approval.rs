use crate::storage_types::DataKey;
use crate::storage_types::{ApprovalAll, ApprovalKey, BALANCE_BUMP_AMOUNT};
use crate::errors::Error;
use soroban_sdk::{Env, Address, panic_with_error};

pub fn read_approval(env: &Env, id: i128) -> Address {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    if let Some(approval) = env.storage().persistent().get::<DataKey, Address>(&key) {
        env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
        approval
    } else {
        panic_with_error!(env, Error::NotFound)
    }
}

pub fn read_approval_all(env: &Env, owner: Address, operator: Address) -> bool {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    if let Some(approval) = env.storage().persistent().get::<DataKey, bool>(&key) {
        env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
        approval
    } else {
        false
    }
}

pub fn write_approval(env: &Env, id: i128, operator: Option<Address>) {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    env.storage().persistent().set(&key, &operator);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

pub fn write_approval_all(env: &Env, owner: Address, operator: Address, approved: bool) {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    env.storage().persistent().set(&key, &approved);
    env.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}