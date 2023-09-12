//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::admin::{has_administrator, read_administrator, write_administrator, get_token, write_token};
use crate::offer::{check_offer, read_offer, write_offer, change_offer};
use crate::error::{Error};
use soroban_sdk::{contract, contractimpl, Address,  Env,  Bytes, BytesN, contracterror,token, Val};
use crate::storage_types::{Offer};

mod nft {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}

pub trait OfferPoolTrait {
    fn initialize(e: Env, admin: Address, token: Address) -> Result<bool, Error>;
    fn create_offer(e: Env, from: Address, offer_id: i128, amount: i128, nft_contract: Address, nft_id:i128) -> Result<bool, Error> ;
    fn expire_offer(e: Env, offer_id: i128) -> Result<bool, Error> ;
    fn get_offer(e: Env, offer_id: i128) -> Result<Offer, Error> ;
    fn accept_offer(e: Env, token: Address, offer_id: i128) -> Result<bool, Error> ;

    // fn approve_offer(e: Env, admin: Address);
}


#[contract]
pub struct OfferPool;



#[contractimpl]
impl OfferPoolTrait for OfferPool {
    fn initialize(e: Env, admin: Address, token: Address) -> Result<bool, Error> {
        
        if has_administrator(&e) {
            Err(Error::AdminExist)
        }else{
            write_administrator(&e, &admin);
            write_token(&e, &token);
            Ok(true)
        }
    }

    fn create_offer(e: Env,from: Address, offer_id: i128, amount: i128, nft_contract: Address, nft_id:i128) -> Result<bool, Error> {
        
        if check_offer(&e ,offer_id) {
           Err(Error::OfferExist)
        }else{
            let token_client = token::Client::new(&e, &get_token(&e));
            token_client.transfer(&from, &e.current_contract_address(), &amount);
            write_offer(&e, offer_id, from, amount, nft_contract, nft_id);
            Ok(true)
        }
    }

    fn expire_offer(e: Env, offer_id: i128) -> Result<bool, Error> {
        
        let offer=read_offer(&e, offer_id);
        match offer {
           // The division was valid
           Some(x) =>{

            if (x.status!=0){
                return Err(Error::OfferChanged)
            }

            let from= x.from;
            let amount=x.amount;

            let token_client = token::Client::new(&e, &get_token(&e));

            token_client.transfer(&e.current_contract_address(), &from,  &amount);
            change_offer(&e, offer_id, 1);
            Ok(true)
           },
           // The division was invalid
           None    => {
               return Err(Error::OfferEmpyt)
           }
       }
    }

    fn get_offer(e: Env, offer_id: i128) -> Result<Offer, Error>{

             let offer=read_offer(&e, offer_id);
             match offer {
                // The division was valid
                Some(x) =>{
                    Ok(x)
                },
                // The division was invalid
                None    => {
                    Err(Error::OfferEmpyt)
                }
            }
        
    }

    fn accept_offer(e: Env, to: Address ,offer_id: i128) -> Result<bool, Error> {
        
        let offer=read_offer(&e, offer_id);
        match offer {
           // The division was valid
           Some(x) =>{

            if (x.status!=0) {
                return Err(Error::OfferChanged)
            }
            let from= x.from;
            let amount=x.amount;
            let nft_contract=x.nft_contract;
            let nft_id=x.nft_id;

            let token_client = token::Client::new(&e, &get_token(&e));
            let nft_client= nft::Client::new(&e, &nft_contract);

            nft_client.transfer(&to ,&from, &nft_id);

            token_client.transfer(&e.current_contract_address(), &to,  &amount);

            change_offer(&e, offer_id, 2);
            Ok(true)
           },
           // The division was invalid
           None    => {
               return Err(Error::OfferEmpyt)
           }
       }
    }


}
