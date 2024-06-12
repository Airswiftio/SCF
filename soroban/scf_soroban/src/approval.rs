use crate::errors::Error;
use crate::storage_types::DataKey;
use crate::storage_types::{ApprovalAll, ApprovalKey};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_approval(env: &Env, id: i128) -> Address {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    if let Some(approval) = env.storage().instance().get::<DataKey, Address>(&key) {
        approval
    } else {
        panic_with_error!(env, Error::NotAuthorized)
    }
}

pub fn read_approval_all(env: &Env, owner: Address, operator: Address) -> bool {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    if let Some(approval) = env.storage().instance().get::<DataKey, bool>(&key) {
        approval
    } else {
        false
    }
}

pub fn write_approval(env: &Env, id: i128, operator: Option<Address>) {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    match operator {
        Some(operator) => env.storage().instance().set(&key, &operator),
        None => env.storage().instance().remove(&key),
    }
}

pub fn write_approval_all(env: &Env, owner: Address, operator: Address, approved: bool) {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    env.storage().instance().set(&key, &approved);
}
