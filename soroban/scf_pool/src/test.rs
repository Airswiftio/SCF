#![cfg(test)]
use crate::contract::{OfferPool, OfferPoolClient};
use crate::error::Error as ContractError;
use crate::test_util::{
    setup_pool, setup_tc, setup_test_token, tc_contract::Error as TCError,
    tc_contract::SplitRequest,
};
use soroban_sdk::{
    map, symbol_short, testutils::Address as _, testutils::Events, vec, Address, Env, Error,
    IntoVal, String,
};

#[test]
fn test_get_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (pool_client, _) = setup_pool(&e, &admin);

    let offer_id = 1;
    let res = pool_client.try_get_offer(&offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferEmpty as u32
        )))
    );
}

#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    client.initialize(&admin);

    assert_eq!(client.get_ext_tokens(), vec![&e]);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice() {
    let e = Env::default();
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
fn test_add_and_remove_ext_tokens() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client_1, _) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);

    // add an ext token
    client.add_ext_token(&token_client_1.address);
    assert_eq!(
        client.get_ext_tokens(),
        vec![&e, token_client_1.address.clone()]
    );

    // adding the same ext token twice should not result in a duplicate value
    client.add_ext_token(&token_client_1.address);
    assert_eq!(
        client.get_ext_tokens(),
        vec![&e, token_client_1.address.clone()]
    );

    // removing a nonexistent ext token should not change the token list
    let (token_client_2, _) = setup_test_token(&e, &admin);
    client.remove_ext_token(&token_client_2.address);
    assert_eq!(
        client.get_ext_tokens(),
        vec![&e, token_client_1.address.clone()]
    );

    // add a second ext token to the whitelist
    client.add_ext_token(&token_client_2.address);
    assert_eq!(
        client.get_ext_tokens(),
        map![
            &e,
            (token_client_1.address.clone(), ()),
            (token_client_2.address.clone(), ())
        ]
        .keys()
    );

    // remove the 1st ext token from the whitelist
    client.remove_ext_token(&token_client_1.address);
    assert_eq!(
        client.get_ext_tokens(),
        vec![&e, token_client_2.address.clone()]
    );
}

#[test]
fn test_create_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address.clone(),
        &600000,
        &tc_client.address,
        &0,
    );

    assert_eq!(offer_id, 0);

    //Test for the event
    //Get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            // Test the event contract address
            assert_eq!(contract_address, contract_id.clone());

            // Test the event topics
            assert_eq!(
                topics,
                (symbol_short!("create"), offerer.clone(), 600000i128).into_val(&e)
            );

            // Test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, 0);
        }
        None => panic!("The event is not published"),
    }

    let offer = pool_client.get_offer(&offer_id);
    //test offer information
    assert_eq!(offer.from, offerer);
    assert_eq!(offer.amount, 600000);
    assert_eq!(offer.tc_contract, tc_client.address);
    assert_eq!(offer.tc_id, 0);
    assert_eq!(offer.status, 0);
    assert_eq!(token_client.balance(&offerer.clone()), 400000);
}

#[test]
fn test_create_offer_unsupported_token() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let res = pool_client.try_create_offer(
        &offerer,
        &token_client.address.clone(),
        &600000,
        &tc_client.address,
        &0,
    );

    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::TokenNotSupported as u32
        )))
    );
}

#[test]
fn test_create_offer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let res = pool_client.try_create_offer(
        &offerer,
        &token_client.address.clone(),
        &2000000,
        &tc_client.address,
        &0,
    );
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_create_offer_disabled_tc() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let supplier2 = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));
    tc_client.split(
        &0,
        &vec![
            &e,
            SplitRequest {
                amount: 800000,
                to: supplier2.clone(),
            },
        ],
    );

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let res = pool_client.try_create_offer(
        &offerer,
        &token_client.address.clone(),
        &600000,
        &tc_client.address,
        &0,
    );
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::TCDisabled as u32
        )))
    );
}

#[test]
fn test_create_offer_nonexistent_tc() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // try to create offer with existing tc contract, but nonexistent tc id
    let res = pool_client.try_create_offer(
        &offerer,
        &token_client.address.clone(),
        &1000000,
        &tc_client.address,
        &0,
    );
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(TCError::NotFound as u32)))
    );

    // try to create offer with nonexistent tc contract
    let random_addr = Address::generate(&e);
    let res = pool_client.try_create_offer(
        &offerer,
        &token_client.address.clone(),
        &1000000,
        &random_addr,
        &0,
    );
    assert!(res.is_err());
}

#[test]
fn test_accept_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // create and accept the offer
    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
    pool_client.accept_offer(&supplier, &offer_id);

    //Test for the event
    //Get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            // Test the event contract address
            assert_eq!(contract_address, contract_id.clone());

            // Test the event topics
            assert_eq!(
                topics,
                (symbol_short!("accept"), supplier.clone()).into_val(&e)
            );

            // Test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, offer_id);
        }
        None => panic!("The event is not published"),
    }

    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 2);
    assert_eq!(token_client.balance(&supplier), 1000000)
}

#[test]
fn test_accept_offer_nonexistent_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (pool_client, _) = setup_pool(&e, &admin);

    let res = pool_client.try_accept_offer(&admin, &123);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferEmpty as u32
        )))
    )
}

#[test]
fn test_accept_offer_not_tc_owner() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // other_user is not the owner of the TC
    let other_user = Address::generate(&e);
    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
    let res = pool_client.try_accept_offer(&other_user, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(TCError::NotOwned as u32)))
    );
}

#[test]
fn test_expire_accepted_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // create and accept the offer
    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
    pool_client.accept_offer(&supplier, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 2);

    // try to expire an accepted offer
    let res = pool_client.try_expire_offer(&admin, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferChanged as u32
        )))
    );
}

#[test]
fn test_expire_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (pool_client, _) = setup_pool(&e, &admin);

    let res = pool_client.try_expire_offer(&admin, &123);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferEmpty as u32
        )))
    )
}

#[test]
fn test_expire_offer_as_admin() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
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
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
    pool_client.expire_offer(&offerer, &offer_id);

    //test for the event
    //get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            //test the event contract address
            assert_eq!(contract_address, contract_id.clone());
            //test the event topics
            assert_eq!(
                topics,
                (symbol_short!("expire"), offerer.clone()).into_val(&e)
            );
            //test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, offer_id);
        }
        None => panic!("the event is not published"),
    }

    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);
}

#[test]
fn test_expire_offer_not_owned() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    let other_user = Address::generate(&e);
    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
    let res = pool_client.try_expire_offer(&other_user, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotAuthorized as u32
        )))
    )
}

#[test]
fn test_accept_expired_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let supplier = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    pool_client.add_ext_token(&token_client.address);

    // setup tc
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    tc_client.mint_original(&supplier, &String::from_str(&e, ""));

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // create and expire the offer
    let offer_id = pool_client.create_offer(
        &offerer,
        &token_client.address,
        &1000000,
        &tc_client.address,
        &0,
    );
    pool_client.expire_offer(&admin, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);

    // try to accept an expired offer
    let res = pool_client.try_accept_offer(&supplier, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferChanged as u32
        )))
    );
}
