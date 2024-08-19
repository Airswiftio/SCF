use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::error::Error;
use crate::event;
use crate::interface::OfferPoolTrait;
use crate::offer::{
    change_offer, increment_supply, read_offer, read_recipient, read_supply, write_offer,
    write_recipient,
};
use crate::pool_token::{has_ext_token, read_ext_tokens, write_ext_tokens};
use crate::storage_types::{Offer, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};

use soroban_sdk::{contract, contractimpl, panic_with_error, token, Address, Env, Vec};

mod tc {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}

#[contract]
pub struct OfferPool;

#[contractimpl]
impl OfferPoolTrait for OfferPool {
    fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
    }

    fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_administrator(&env)
    }

    fn set_admin(env: Env, new_admin: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    fn add_ext_token(e: Env, ext_token_address: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let mut token_list = read_ext_tokens(&e);
        if token_list.contains_key(ext_token_address.clone()) {
            return;
        }
        token_list.set(ext_token_address.clone(), ());
        write_ext_tokens(&e, token_list);
    }

    fn remove_ext_token(e: Env, ext_token_address: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let mut token_list = read_ext_tokens(&e);
        if !token_list.contains_key(ext_token_address.clone()) {
            return;
        }
        token_list.remove(ext_token_address.clone());
        write_ext_tokens(&e, token_list);
    }

    fn get_ext_tokens(e: Env) -> Vec<Address> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let token_list = read_ext_tokens(&e);
        token_list.keys()
    }

    fn create_offer(
        e: Env,
        from: Address,
        ext_token: Address,
        amount: i128,
        fee: i128,
        tc_contract: Address,
        tc_id: i128,
    ) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if !has_ext_token(&e, ext_token.clone()) {
            panic_with_error!(&e, Error::TokenNotSupported);
        }
        // Transfer the offer amount to the contract address until the offer is accepted or expired.
        let token_client = token::Client::new(&e, &ext_token);
        let tc_client = tc::Client::new(&e, &tc_contract);

        // calling the contract to check if TC is disabled or already loaned
        if tc_client.is_disabled(&tc_id) {
            panic_with_error!(&e, Error::TCDisabled);
        }
        if tc_client.loan_status(&tc_id) != 0 {
            panic_with_error!(&e, Error::TCAlreadyLoaned);
        }

        // TODO: this only works if the TC amount is expressed in the same currency as the ext_token.
        let tc_amount = i128::from(tc_client.amount(&tc_id)) * 10i128.pow(token_client.decimals());
        let remainder = tc_amount - amount - fee;
        if remainder < 0 {
            panic_with_error!(&e, Error::InvalidAmount);
        }

        from.require_auth();
        token_client.transfer(&from, &e.current_contract_address(), &amount);

        let offer_id = read_supply(&e);

        write_offer(
            &e,
            offer_id,
            from.clone(),
            ext_token,
            amount,
            fee,
            remainder,
            tc_contract,
            tc_id,
        );

        increment_supply(&e);
        event::create_offer(&e, from, offer_id, amount, fee);
        return offer_id;
    }

    // Cancels an offer and returns the offered amount to the owner. Callable by the admin or offer owner.
    fn expire_offer(e: Env, from: Address, offer_id: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        match read_offer(&e, offer_id) {
            Some(offer) => {
                if offer.status != 0 {
                    panic_with_error!(&e, Error::OfferChanged);
                }
                // check that 'from' matches either the admin or the offer owner
                let admin = read_administrator(&e);
                let offer_from = offer.from;
                if (from != admin) && (from != offer_from) {
                    panic_with_error!(&e, Error::NotAuthorized);
                }

                // transfer the offer amount from the contract address back to the offer owner
                from.require_auth();
                let amount = offer.amount;
                let token_client = token::Client::new(&e, &offer.pool_token);

                token_client.transfer(&e.current_contract_address(), &offer_from, &amount);
                change_offer(&e, offer_id, 1);
                event::expire_offer(&e, from, offer_id);
            }
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    fn get_offer(e: Env, offer_id: i128) -> Offer {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => return x,
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    // On accepting an offer, the offered amount in tokens is transferred from to contract address to 'to' and the TC is transferred to the offer creator.
    fn accept_offer(e: Env, to: Address, offer_id: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        match read_offer(&e, offer_id) {
            Some(offer) => {
                if offer.status != 0 {
                    panic_with_error!(&e, Error::OfferChanged);
                }
                let from = offer.from;
                let amount = offer.amount;
                let tc_contract = offer.tc_contract;
                let tc_id = offer.tc_id;

                let token_client = token::Client::new(&e, &offer.pool_token);
                let tc_client = tc::Client::new(&e, &tc_contract);
                if tc_client.is_disabled(&tc_id) {
                    panic_with_error!(&e, Error::TCDisabled);
                }
                if tc_client.loan_status(&tc_id) != 0 {
                    panic_with_error!(&e, Error::TCAlreadyLoaned);
                }
                to.require_auth();
                tc_client.transfer(&to, &from, &tc_id);

                token_client.transfer(&e.current_contract_address(), &to, &amount);

                change_offer(&e, offer_id, 2);
                tc_client.set_loan_status(&tc_id, &1);
                write_recipient(&e, offer_id, to.clone());
                event::accept_offer(&e, to, offer_id);
            }
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    fn close_offer(e: Env, offer_id: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        match read_offer(&e, offer_id) {
            Some(offer) => {
                if offer.status != 2 {
                    panic_with_error!(&e, Error::OfferChanged);
                }
                let recipient = read_recipient(&e, offer_id);
                offer.from.require_auth();

                let token_client = token::Client::new(&e, &offer.pool_token);
                token_client.transfer(&offer.from, &recipient, &offer.remainder);

                let tc_contract = offer.tc_contract;
                let tc_id = offer.tc_id;
                let tc_client = tc::Client::new(&e, &tc_contract);
                if tc_client.is_disabled(&tc_id) {
                    panic_with_error!(&e, Error::TCDisabled);
                }
                if tc_client.loan_status(&tc_id) != 1 {
                    panic_with_error!(&e, Error::TCNotLoaned);
                }
                tc_client.set_loan_status(&tc_id, &2);

                change_offer(&e, offer_id, 3);
                event::close_offer(&e, offer.from, offer_id, offer.remainder);
            }
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    fn recipient(e: Env, offer_id: i128) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_recipient(&e, offer_id)
    }
}
