use crate::{
    error::Error,
    storage_types::{DataKey, Offer, OFFER_BUMP_AMOUNT, OFFER_LIFETIME_THRESHOLD},
};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_offer(e: &Env, offer_id: i128) -> Option<Offer> {
    let key = DataKey::Offer(offer_id);
    if let Some(offer) = e.storage().persistent().get::<DataKey, Offer>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
        Some(offer)
    } else {
        None
    }
}

pub fn read_supply(e: &Env) -> i128 {
    let key = DataKey::Supply;
    match e.storage().instance().get::<DataKey, i128>(&key) {
        Some(offer_id) => offer_id,
        None => 0,
    }
}

pub fn increment_supply(e: &Env) {
    let key = DataKey::Supply;
    e.storage().instance().set(&key, &(read_supply(&e) + 1));
}

pub fn write_offer(
    e: &Env,
    offer_id: i128,
    from: Address,
    pool_token: Address,
    amount: i128,
    fee: i128,
    remainder: i128,
    tc_contract: Address,
    tc_id: i128,
) {
    let input_offer = Offer {
        from,
        pool_token,
        amount,
        fee,
        remainder,
        tc_contract,
        tc_id,
        status: 0,
    };
    let key = DataKey::Offer(offer_id);
    e.storage().persistent().set(&key, &input_offer);
    e.storage()
        .persistent()
        .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
}

pub fn change_offer(e: &Env, offer_id: i128, status: i128) -> bool {
    let key = DataKey::Offer(offer_id);
    if let Some(offer) = e.storage().persistent().get::<DataKey, Offer>(&key) {
        let mut new_offer = offer;
        new_offer.status = status;
        e.storage().persistent().set(&key, &new_offer);
        e.storage()
            .persistent()
            .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
        return true;
    } else {
        return false;
    }
}

pub fn read_recipient(e: &Env, offer_id: i128) -> Address {
    let key = DataKey::Recipient(offer_id);
    if let Some(data) = e.storage().persistent().get::<DataKey, Address>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
        data
    } else {
        panic_with_error!(&e, Error::RecipientNotFound)
    }
}

pub fn write_recipient(e: &Env, offer_id: i128, recipient: Address) {
    let key = DataKey::Recipient(offer_id);
    e.storage().persistent().set(&key, &recipient);
    e.storage()
        .persistent()
        .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
}
