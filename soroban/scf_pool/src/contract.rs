use crate::admin::{
    get_token, has_administrator, read_administrator, write_administrator, write_token,
};
use crate::error::Error;
use crate::offer::{change_offer, check_offer, read_offer, write_offer};
use crate::storage_types::{Offer, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{contract, contractimpl, token, Address, Env};

mod tc {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}

pub trait OfferPoolTrait {
    fn initialize(e: Env, admin: Address, token: Address) -> Result<bool, Error>;
    fn create_offer(
        e: Env,
        from: Address,
        offer_id: i128,
        amount: i128,
        tc_contract: Address,
        tc_id: i128,
    ) -> Result<bool, Error>;
    fn expire_offer(e: Env, from: Address, offer_id: i128) -> Result<bool, Error>;
    fn get_offer(e: Env, offer_id: i128) -> Result<Offer, Error>;
    fn accept_offer(e: Env, token: Address, offer_id: i128) -> Result<bool, Error>;

    // fn approve_offer(e: Env, admin: Address);
}

#[contract]
pub struct OfferPool;

#[contractimpl]
impl OfferPoolTrait for OfferPool {
    fn initialize(e: Env, admin: Address, token: Address) -> Result<bool, Error> {
        if has_administrator(&e) {
            Err(Error::AdminExist)
        } else {
            write_administrator(&e, &admin);
            write_token(&e, &token);
            Ok(true)
        }
    }

    /// Creates an offer pointing to a specific TC.
    fn create_offer(
        e: Env,
        from: Address,
        offer_id: i128,
        amount: i128,
        tc_contract: Address,
        tc_id: i128,
    ) -> Result<bool, Error> {
        if check_offer(&e, offer_id) {
            Err(Error::OfferExist)
        } else {
            e.storage()
                .instance()
                .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
            // Transfer the offer amount to the contract address until the offer is accepted or expired.
            let token_client = token::Client::new(&e, &get_token(&e));
            from.require_auth();
            token_client.transfer(&from, &e.current_contract_address(), &amount);
            write_offer(&e, offer_id, from, amount, tc_contract, tc_id);
            Ok(true)
        }
    }

    // Cancels an offer and returns the offered amount to the owner. Callable by the admin or offer owner.
    fn expire_offer(e: Env, from: Address, offer_id: i128) -> Result<bool, Error> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => {
                if x.status != 0 {
                    return Err(Error::OfferChanged);
                }
                // check that 'from' matches either the admin or the offer owner
                let admin = read_administrator(&e);
                let offer_from = x.from;
                if (from != admin) && (from != offer_from) {
                    return Err(Error::NotAuthorized);
                }

                // transfer the offer amount from the contract address back to the offer owner
                from.require_auth();
                let amount = x.amount;
                let token_client = token::Client::new(&e, &get_token(&e));

                token_client.transfer(&e.current_contract_address(), &offer_from, &amount);
                change_offer(&e, offer_id, 1);
                Ok(true)
            }
            None => return Err(Error::OfferEmpty),
        }
    }

    fn get_offer(e: Env, offer_id: i128) -> Result<Offer, Error> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => Ok(x),
            None => Err(Error::OfferEmpty),
        }
    }

    // On accepting an offer, the offered amount in tokens is transferred from to contract address to 'to' and the TC is transferred to the offer creator.
    fn accept_offer(e: Env, to: Address, offer_id: i128) -> Result<bool, Error> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => {
                if x.status != 0 {
                    return Err(Error::OfferChanged);
                }
                let from = x.from;
                let amount = x.amount;
                let tc_contract = x.tc_contract;
                let tc_id = x.tc_id;

                let token_client = token::Client::new(&e, &get_token(&e));
                let tc_client = tc::Client::new(&e, &tc_contract);

                to.require_auth();
                tc_client.transfer(&to, &from, &tc_id);

                token_client.transfer(&e.current_contract_address(), &to, &amount);

                change_offer(&e, offer_id, 2);
                Ok(true)
            }
            None => return Err(Error::OfferEmpty),
        }
    }
}
