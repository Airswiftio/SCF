use soroban_sdk::{contracttype, Address};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

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
    Approval(ApprovalKey),
    Owner(i128),
    Disabled(i128),
    Supply,
    SubTCInfo(i128),
    VC(i128),
    OrderInfo,
    Expired,
    Paid,
    ExternalToken,
    Recipient(i128),
}

#[derive(Clone)]
#[contracttype]
pub struct SubTC {
    pub root: i128,
    pub amount: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitRequest {
    pub amount: u32,
    pub to: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalToken {
    pub contract_addr: Address,
    pub decimals: u32,
}
