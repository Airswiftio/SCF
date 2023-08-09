use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days

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
    Expired,
    Paid,
}

#[derive(Clone)]
#[contracttype]
pub struct SubNFT {
    pub root: i128,
    pub amount: u32,
}
