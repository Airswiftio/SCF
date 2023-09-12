use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const OFFER_BUMP_AMOUNT: u32 = 518400; // 30 days

#[derive(Clone)]
#[contracttype]
pub struct Offer {
    pub from: Address,
    pub amount: i128,
    pub nft_contract: Address,
    pub nft_id:i128,
    pub status: i128,
}
// pub struct OfferID {
//     pub from: str,
// }

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(i128),
    TokenContract,
    Admin,
}