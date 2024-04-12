#![cfg(test)]
use crate::test_util::{setup_pool, setup_tc, setup_test_token};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_get_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);

    let offer_id = 1;
    pool.get_offer(&offer_id);
}

#[test]
fn test_create_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
    let offer = pool.get_offer(&offer_id);
    assert_eq!(offer.from, offerer);
    assert_eq!(offer.amount, 1000000);
    assert_eq!(offer.tc_contract, tc_contract.address);
    assert_eq!(offer.tc_id, 1);
    assert_eq!(offer.status, 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #10)")]
fn test_create_offer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    // create offer when the offerer does not hold any test tokens
    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_create_offer_duplicate() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    // try to create two offers with the same offer_id
    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
    pool.create_offer(&offerer, &offer_id, &2000000, &tc_contract.address, &2);
}

#[test]
fn test_accept_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_contract.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and accept the offer
    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &0);
    pool.accept_offer(&tc_holder, &offer_id);
    let offer = pool.get_offer(&offer_id);
    assert_eq!(offer.status, 2);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_accept_offer_nonexistent_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);

    pool.accept_offer(&admin, &123);
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
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    // try to accept an offer for a TC that was never minted
    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &0);
    pool.accept_offer(&admin, &offer_id);
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
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_contract.mint_original(&tc_holder, &String::from_str(&e, ""));

    // other_user is not the owner of the TC
    let offer_id = 1;
    let other_user = Address::generate(&e);
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &0);
    pool.accept_offer(&other_user, &offer_id);
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
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_contract.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and accept the offer
    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &0);
    pool.accept_offer(&tc_holder, &offer_id);
    let offer = pool.get_offer(&offer_id);
    assert_eq!(offer.status, 2);

    // try to expire an accepted offer
    pool.expire_offer(&admin, &offer_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_expire_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);

    pool.expire_offer(&admin, &123);
}

#[test]
fn test_expire_offer_as_admin() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
    pool.expire_offer(&admin, &offer_id);
    let offer = pool.get_offer(&offer_id);
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
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
    pool.expire_offer(&offerer, &offer_id);
    let offer = pool.get_offer(&offer_id);
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
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    let offer_id = 1;
    let other_user = Address::generate(&e);
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
    pool.expire_offer(&other_user, &offer_id);
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
    let pool = setup_pool(&e, &admin, &token_client.address);
    let tc_contract = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_contract.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and expire the offer
    let offer_id = 1;
    pool.create_offer(&offerer, &offer_id, &1000000, &tc_contract.address, &1);
    pool.expire_offer(&admin, &offer_id);
    let offer = pool.get_offer(&offer_id);
    assert_eq!(offer.status, 1);

    // try to accept an expired offer
    pool.accept_offer(&tc_holder, &offer_id);
}
