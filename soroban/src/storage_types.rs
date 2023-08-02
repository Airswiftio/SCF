use soroban_sdk::{contracttype, Address, Vec};

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
    Nonce(Address),
    Minted(Address),
    Admin,
    Name,
    Symbol,
    URI(i128),
    Approval(ApprovalKey),
    Owner(i128),
    Disabled(i128),
    Supply,
    SubNFTInfo(i128),
    OrderInfo,
}

#[derive(Clone)]
#[contracttype]
pub struct SubNFT{
    pub root: i128,
    pub amount: u32,
}