use crate::admin::{ has_administrator, read_administrator, write_administrator};
use crate::error::Error;
use crate::offer::{change_offer, check_offer, read_offer, write_offer};
use crate::storage_types::{Offer, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD, TokenInfo};
use crate::pool_token::{create_contract, read_pool_token, write_pool_token};
use crate::ext_token::{self, read_ext_token, write_ext_token};
use crate::interface::{OfferPoolTrait};

use soroban_sdk::{ contract, contractimpl, panic_with_error, token, vec, Address, BytesN, Env, IntoVal, Symbol, Val,};

mod tc {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}


#[contract]
pub struct OfferPool;

#[contractimpl]
impl OfferPoolTrait for OfferPool {
    fn initialize(
        e: Env,
        admin: Address, 
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,
    ){
        if has_administrator(&e) {
            panic!("already initialized")
        } 
        write_administrator(&e, &admin);
        if ext_token_decimals > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }
        let pool_token_contract = create_contract(&e, token_wasm_hash);
        e.invoke_contract::<Val>(
            &pool_token_contract,
            &"initialize".into_val(&e),
            vec![
                &e,
                e.current_contract_address().into_val(&e),
                7u32.into_val(&e),
                "Pool Token".into_val(&e),
                "SPT".into_val(&e),
            ],
        );
        
        write_pool_token(
            &e,
            TokenInfo {
                address: pool_token_contract,
                decimals: 7,
            },
        );
        write_ext_token(
            &e,
            TokenInfo {
                address: ext_token_address,
                decimals: ext_token_decimals,
            },
        );
    }
    
    fn deposit(e: Env, from: Address, amount: i128) {
        from.require_auth();
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Transfer select token from "from" to the contract address
        let ext_token=read_ext_token(&e);
        let client=token::Client::new(&e, &ext_token.address);
        client.transfer(&from,&e.current_contract_address(), &amount);

        //Mint the equal amount number of liquidity tokens to from
        token::StellarAssetClient::new(&e, &read_pool_token(&e).address).mint(&from,&amount);
    }

    fn withdraw(e: Env, from: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Burn the specified number of liquidity tokens from "from"
        token::Client::new(&e, &read_pool_token(&e).address).burn(&from, &amount);

        // Transfer USDC from the contract address to "from"
        token::Client::new(&e, &read_ext_token(&e).address).transfer(
            &e.current_contract_address(),
            &from,
            &amount,
        );
    }
    /// Creates an offer pointing to a specific TC.
    fn create_offer(
        e: Env,
        from: Address,
        offer_id: i128,
        amount: i128,
        tc_contract: Address,
        tc_id: i128,
    ){
        if check_offer(&e, offer_id) {
            panic_with_error!(&e, Error::OfferEmpty);
        } else {
            e.storage()
                .instance()
                .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
            // Transfer the offer amount to the contract address until the offer is accepted or expired.
            let pool_token=read_pool_token(&e);
            let token_client = token::Client::new(&e, &pool_token.address);
            from.require_auth();
            token_client.transfer(&from, &e.current_contract_address(), &amount);
            write_offer(&e, offer_id, from, amount, tc_contract, tc_id);
        }
    }

    // Cancels an offer and returns the offered amount to the owner. Callable by the admin or offer owner.
    fn expire_offer(e: Env, from: Address, offer_id: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => {
                if x.status != 0 {
                    panic_with_error!(&e, Error::OfferChanged);
                }
                // check that 'from' matches either the admin or the offer owner
                let admin = read_administrator(&e);
                let offer_from = x.from;
                if (from != admin) && (from != offer_from) {
                    panic_with_error!(&e, Error::NotAuthorized);
                }

                // transfer the offer amount from the contract address back to the offer owner
                from.require_auth();
                let amount = x.amount;
                let pool_token=read_pool_token(&e);
                let token_client = token::Client::new(&e, &pool_token.address);

                token_client.transfer(&e.current_contract_address(), &offer_from, &amount);
                change_offer(&e, offer_id, 1);
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
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => {
                if x.status != 0 {
                    panic_with_error!(&e,Error::OfferChanged);
                }
                let from = x.from;
                let amount = x.amount;
                let tc_contract = x.tc_contract;
                let tc_id = x.tc_id;

                let pool_token=read_pool_token(&e);
                let token_client = token::Client::new(&e, &pool_token.address);
                let tc_client = tc::Client::new(&e, &tc_contract);

                to.require_auth();
                tc_client.transfer(&to, &from, &tc_id);

                token_client.transfer(&e.current_contract_address(), &to, &amount);

                change_offer(&e, offer_id, 2);
            }
            None => panic_with_error!(&e,Error::OfferEmpty),
        }
    }
    
    fn get_liquidity_token(e: Env) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_pool_token(&e).address
    }

    fn get_ext_token(e: Env) -> (Address, u32) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let ext_token = read_ext_token(&e);
        (ext_token.address, ext_token.decimals)
    }

}
