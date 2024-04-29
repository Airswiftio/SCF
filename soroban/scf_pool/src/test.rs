#![cfg(test)]
use crate::test_util::{setup_pool, setup_tc, setup_test_token, install_token_wasm};
use crate::contract::{OfferPool, OfferPoolClient};
use crate::error::{Error as ContractError};
use soroban_sdk::{testutils::Address as _, Address, Env, String, token, Error};


#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_get_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let (pool_client,_) = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let offer_id = 1;
    pool_client.get_offer(&offer_id);
}


#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
    );

    assert_eq!(
        client.get_ext_token(),
        (token_client.address.clone(), token_client.decimals())
    );
    assert_eq!(client.try_get_liquidity_token().is_ok(), true);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice() {
    let e = Env::default();
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals());

    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals());
}

#[test]
fn test_deposit() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &600000);
    assert_eq!(token_client.balance(&user.clone()), 400000);
    assert_eq!(
        liquidity_token_client.balance(&user.clone()),
        600000
    );
}

#[test]
fn test_deposit_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    let res = client.try_deposit(&user.clone(), &1);
    assert_eq!(res, Err(Ok(Error::from_contract_error(10))));
}

#[test]
fn test_withdraw() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &600000);
    client.withdraw(&user.clone(), &100000);

    assert_eq!(token_client.balance(&user.clone()), 500000);
    assert_eq!(
        liquidity_token_client.balance(&user.clone()),
        500000
    );
}

#[test]
fn test_withdraw_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    let res = client.try_withdraw(&user.clone(), &1);
    assert_eq!(res.is_err(), true);
}


#[test]
fn test_create_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    

    //mint USDC token to offerer
    token_admin_client.mint(&offerer, &1000000);
    
    //offerer deposite USDC and exchange to PL token
    pool_client.deposit(&offerer.clone(), &1000000);

    let offer_id = 1;

    pool_client.create_offer(&offerer, &offer_id, &600000, &tc_client.address, &1);
    let offer = pool_client.get_offer(&offer_id);
    //test offer information
    assert_eq!(offer.from, offerer);
    assert_eq!(offer.amount, 600000);
    assert_eq!(offer.tc_contract, tc_client.address);
    assert_eq!(offer.tc_id, 1);
    assert_eq!(offer.status, 0);
    assert_eq!(
        liquidity_token_client.balance(&offerer.clone()),
        400000
    );
}

#[test]
fn test_create_offer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    

    //mint USDC token to offerer
    token_admin_client.mint(&offerer, &1000000);
    
    //offerer deposite USDC and exchange to PL token
    pool_client.deposit(&offerer.clone(), &1000000);

    let offer_id = 1;

    let res =pool_client.try_create_offer(&offerer, &offer_id, &2000000, &tc_client.address, &1);
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_create_offer_duplicate() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    

    //mint USDC token to offerer
    token_admin_client.mint(&offerer, &2000000);
    
    //offerer deposite USDC and exchange to PL token
    pool_client.deposit(&offerer.clone(), &2000000);

    let offer_id = 1;

    let res =pool_client.try_create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &1);
    assert_eq!(res.is_ok() , true);
    assert_eq!(liquidity_token_client.balance(&offerer.clone()), 1000000);
    let res =pool_client.try_create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &1);
    assert_eq!(res , Err(Ok(Error::from_contract_error(ContractError::OfferEmpty as u32))));
}

#[test]
fn test_accept_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    // setup TC
    tc_client.mint_original(&buyer, &String::from_str(&e, ""));

    // create and accept the offer
    let offer_id = 1;
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &0);
    pool_client.accept_offer(&buyer, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 2);
    assert_eq!(liquidity_token_client.balance(&buyer),1000000)
}

#[test]
fn test_accept_offer_nonexistent_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let (pool_client,_) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());

    let res=pool_client.try_accept_offer(&admin, &123);
    assert_eq!(res, Err(Ok(Error::from_contract_error(
        ContractError::OfferEmpty as u32
    ))))

}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_accept_offer_nonexistent_tc() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    // try to accept an offer for a TC that was never minted
    let offer_id = 1;
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &0);
    pool_client.accept_offer(&admin, &offer_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_accept_offer_not_tc_owner() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_client.mint_original(&tc_holder, &String::from_str(&e, ""));

    // other_user is not the owner of the TC
    let offer_id = 1;
    let other_user = Address::generate(&e);
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &0);
    pool_client.accept_offer(&other_user, &offer_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_expire_accepted_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_client.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and accept the offer
    let offer_id = 1;
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &0);
    pool_client.accept_offer(&tc_holder, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 2);

    // try to expire an accepted offer
    pool_client.expire_offer(&admin, &offer_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_expire_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());

    pool_client.expire_offer(&admin, &123);
}

#[test]
fn test_expire_offer_as_admin() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    let offer_id = 1;
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &1);
    pool_client.expire_offer(&admin, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);
}

#[test]
fn test_expire_offer_as_owner() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    let offer_id = 1;
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &1);
    pool_client.expire_offer(&offerer, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_expire_offer_not_owned() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    let offer_id = 1;
    let other_user = Address::generate(&e);
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &1);
    pool_client.expire_offer(&other_user, &offer_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_accept_expired_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, liquidity_token_client) = setup_pool(&e, &admin, &token_client.address,&token_client.decimals());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_client.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and expire the offer
    let offer_id = 1;
    pool_client.create_offer(&offerer, &offer_id, &1000000, &tc_client.address, &1);
    pool_client.expire_offer(&admin, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);

    // try to accept an expired offer
    pool_client.accept_offer(&tc_holder, &offer_id);
}
