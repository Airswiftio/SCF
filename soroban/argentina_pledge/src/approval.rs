use crate::errors::Error;
use crate::storage_types::DataKey;
use crate::storage_types::{ApprovalAll, ApprovalKey};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_approval(e: &Env, id: i128) -> Address {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    if let Some(approval) = e.storage().instance().get::<DataKey, Address>(&key) {
        approval
    } else {
        panic_with_error!(e, Error::NotAuthorized)
    }
}

pub fn read_approval_all(e: &Env, owner: Address, operator: Address) -> bool {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    if let Some(approval) = e.storage().instance().get::<DataKey, bool>(&key) {
        approval
    } else {
        false
    }
}

pub fn write_approval(e: &Env, id: i128, operator: Option<Address>) {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    match operator {
        Some(operator) => e.storage().instance().set(&key, &operator),
        None => e.storage().instance().remove(&key),
    }
}

pub fn write_approval_all(e: &Env, owner: Address, operator: Address, approved: bool) {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    e.storage().instance().set(&key, &approved);
}
