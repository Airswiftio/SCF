#![cfg(test)]
use soroban_sdk::{testutils::Address as _, token, Address, Env, Error, IntoVal, String};

use crate::{
    contract::LiquidityPoolClient,
    errors::Error as ContractError,
    test_util::{install_token_wasm, setup_pool, setup_tc, setup_test_token},
    LiquidityPool,
};

#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(&e, &contract_id);

    let admin = Address::random(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
        &2,
    );

    assert_eq!(
        client.get_ext_token(),
        (token_client.address.clone(), token_client.decimals())
    );
    assert_eq!(client.get_pool_rate(), 2);
    assert_eq!(client.try_get_liquidity_token().is_ok(), true);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(&e, &contract_id);

    let admin = Address::random(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
        &2,
    );

    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
        &2,
    );
}

#[test]
fn test_deposit() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::random(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &600000);
    assert_eq!(token_client.balance(&user.clone()), 400000);
    assert_eq!(
        token::Client::new(&e, &client.get_liquidity_token()).balance(&user.clone()),
        600000
    );
}

#[test]
fn test_deposit_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::random(&e);
    let res = client.try_deposit(&user.clone(), &1);
    assert_eq!(res, Err(Ok(Error::from_contract_error(10))));
}

#[test]
fn test_withdraw() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::random(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &600000);
    client.withdraw(&user.clone(), &100000);

    assert_eq!(token_client.balance(&user.clone()), 500000);
    assert_eq!(
        token::Client::new(&e, &client.get_liquidity_token()).balance(&user.clone()),
        500000
    );
}

#[test]
fn test_withdraw_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::random(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    let res = client.try_withdraw(&user.clone(), &1);
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_create_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::random(&e);
    let creditor = Address::random(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(
        &1000000,
        &1641024000,
        &String::from_slice(&e, "a"),
        &String::from_slice(&e, "b"),
        &String::from_slice(&e, "c"),
    );
    tc_client.pledge(&borrower.clone(), &0);
    client.deposit(&creditor.clone(), &10000000000000);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(client.get_loan_rate(&loan_id), 0);
    assert_eq!(client.get_loan_creditor(&loan_id), creditor.clone());
    assert_eq!(client.get_loan_tc(&loan_id), (tc_client.address.clone(), 0));
    assert_eq!(client.get_loan_amount(&loan_id), 1000000);
    assert_eq!(client.get_loan_status(&loan_id), 0);
}

#[test]
fn test_create_loan_offer_duplicate() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::random(&e);
    let creditor = Address::random(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(
        &1000000,
        &1641024000,
        &String::from_slice(&e, "a"),
        &String::from_slice(&e, "b"),
        &String::from_slice(&e, "c"),
    );
    tc_client.pledge(&borrower.clone(), &0);
    client.deposit(&creditor.clone(), &10000000000000);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    let res = client.try_create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotEmpty as u32
        )))
    );
}

#[test]
fn test_create_loan_offer_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::random(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::random(&e);
    let creditor = Address::random(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    tc_client.mint(
        &1000000,
        &1641024000,
        &String::from_slice(&e, "a"),
        &String::from_slice(&e, "b"),
        &String::from_slice(&e, "c"),
    );
    tc_client.pledge(&borrower.clone(), &0);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    let res = client.try_create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(res, Err(Ok(Error::from_contract_error(10))));
}
