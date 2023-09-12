use crate::storage_types::{DataKey, OFFER_BUMP_AMOUNT, Offer};
use soroban_sdk::{Address, Env};


pub fn read_offer(e: &Env, offer_id: i128) -> Option<Offer> {
    let key = DataKey::Offer(offer_id);
    if let Some(offer) = e.storage().persistent().get::<DataKey, Offer>(&key) {
        e.storage().persistent().bump(&key, OFFER_BUMP_AMOUNT);
        Some(offer)
    } else {
        None
    }
}

pub fn check_offer(e: &Env, offer_id: i128) -> bool {
    let key = DataKey::Offer(offer_id);
    e.storage().persistent().has(&key)
}

pub fn write_offer(e: &Env, offer_id: i128, from: Address, amount: i128, nft_contract: Address, nft_id:i128) {
    let input_offer= Offer {
        from:           from,
        amount:         amount,
        nft_contract:   nft_contract,
        nft_id:         nft_id,
        status: 0,
    };
    let key = DataKey::Offer(offer_id);
    e.storage().persistent().set(&key, &input_offer);
    e.storage().persistent().bump(&key, OFFER_BUMP_AMOUNT);
}

pub fn change_offer(e: &Env, offer_id: i128, status:i128) -> bool{

    let key = DataKey::Offer(offer_id);
    if let Some(offer) = e.storage().persistent().get::<DataKey, Offer>(&key) {
        let mut new_offer= offer;
        new_offer.status=status;
        e.storage().persistent().set(&key, &new_offer);
        e.storage().persistent().bump(&key, OFFER_BUMP_AMOUNT);
        return true;
    } else {
        return false
    }
}

// pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
//     let balance = read_balance(e, addr.clone());
//     if !is_authorized(e, addr.clone()) {
//         panic!("can't receive when deauthorized");
//     }
//     write_balance(e, addr, balance + amount);
// }

// pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
//     let balance = read_balance(e, addr.clone());
//     if !is_authorized(e, addr.clone()) {
//         panic!("can't spend when deauthorized");
//     }
//     if balance < amount {
//         panic!("insufficient balance");
//     }
//     write_balance(e, addr, balance - amount);
// }

// pub fn is_authorized(e: &Env, addr: Address) -> bool {
//     let key = DataKey::State(addr);
//     if let Some(state) = e.storage().persistent().get::<DataKey, bool>(&key) {
//         state
//     } else {
//         true
//     }
// }

// pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
//     let key = DataKey::State(addr);
//     e.storage().persistent().set(&key, &is_authorized);
// }
