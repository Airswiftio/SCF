use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct ApprovalAll {
    pub operator: Address,
    pub owner: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum ApprovalKey {
    All(ApprovalAll),
    ID(i128),
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance(Address),
    Nonce(Address),
    Minted(Address),
    Admin,
    Name,
    Symbol,
    URI(i128),
    Approval(ApprovalKey),
    Owner(i128),
    Supply,
}