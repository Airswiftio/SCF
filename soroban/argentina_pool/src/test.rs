#![cfg(test)]
use soroban_sdk::{map, testutils::Address as _, vec, Address, BytesN, Env, Error, Vec};

use crate::{
    contract::LiquidityPoolClient,
    errors::Error as ContractError,
    loan::LoanStatus,
    test_util::{setup_pool, setup_tc, setup_test_token},
    LiquidityPool,
};

#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    client.initialize(&admin, &token_client.address, &token_client.decimals(), &2);

    assert_eq!(
        client.get_ext_token(),
        (token_client.address.clone(), token_client.decimals())
    );
    assert_eq!(client.get_pool_fee(), 2);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    client.initialize(&admin, &token_client.address, &token_client.decimals(), &2);

    client.initialize(&admin, &token_client.address, &token_client.decimals(), &2);
}

#[test]
fn test_tc_whitelist() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    // initial whitelist should be empty
    let mut whitelist = client.get_whitelisted_tcs();
    assert_eq!(whitelist, Vec::<Address>::new(&e));

    // add a TC to the whitelist
    client.add_whitelisted_tc(&tc_client.address);
    whitelist = client.get_whitelisted_tcs();
    assert_eq!(whitelist, vec![&e, tc_client.address.clone()]);

    // adding the same TC to the whitelist twice should not result in a duplicate value
    client.add_whitelisted_tc(&tc_client.address);
    whitelist = client.get_whitelisted_tcs();
    assert_eq!(whitelist, vec![&e, tc_client.address.clone()]);

    // removing a nonexistent TC should not change the whitelist
    let tc_client_2 = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    client.remove_whitelisted_tc(&tc_client_2.address);
    assert_eq!(whitelist, vec![&e, tc_client.address.clone()]);

    // add a second TC to the whitelist
    client.add_whitelisted_tc(&tc_client_2.address);
    whitelist = client.get_whitelisted_tcs();
    assert_eq!(
        whitelist,
        map![
            &e,
            (tc_client.address.clone(), ()),
            (tc_client_2.address.clone(), ())
        ]
        .keys(),
    );

    // remove the 1st TC from the whitelist
    client.remove_whitelisted_tc(&tc_client.address);
    whitelist = client.get_whitelisted_tcs();
    assert_eq!(whitelist, vec![&e, tc_client_2.address.clone()]);
}

#[test]
fn test_create_loan_offer_no_whitelist() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let creditor = Address::generate(&e);

    // call should fail because TC was not whitelisted
    let res = client.try_create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::TCNotWhitelisted as u32
        )))
    );
}

#[test]
fn test_create_loan_offer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    client.add_whitelisted_tc(&tc_client.address);

    // call should fail because creditor does not have enough token
    let res = client.try_create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    assert_eq!(res, Err(Ok(Error::from_contract_error(10))));
}

#[test]
fn test_create_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &20000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    client.add_whitelisted_tc(&tc_client.address);

    // successful call
    let mut loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    assert_eq!(loan_id, 0);
    assert_eq!(client.get_loan_fee(&loan_id), 0);
    assert_eq!(client.get_loan_creditor(&loan_id), creditor.clone());
    assert_eq!(client.get_loan_tc(&loan_id), (tc_client.address.clone(), 0));
    assert_eq!(client.get_loan_amount(&loan_id), 1000000);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Pending as u32);
    assert_eq!(token_client.balance(&creditor), 10000000000000);
    assert_eq!(token_client.balance(&client.address), 10000000000000);

    // create another loan offer, returned loan id should be incremented
    loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    assert_eq!(loan_id, 1);
}

#[test]
fn test_cancel_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);

    client.add_whitelisted_tc(&tc_client.address);
    let loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Pending as u32);
    assert_eq!(token_client.balance(&creditor.clone()), 0);
    client.cancel_loan_offer(&loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Closed as u32);
    assert_eq!(token_client.balance(&creditor.clone()), 10000000000000);
}

#[test]
fn test_accept_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.add_whitelisted_tc(&tc_client.address);
    let loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    client.accept_loan_offer(&borrower.clone(), &loan_id);
    assert_eq!(client.get_loan_borrower(&loan_id), borrower.clone());
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Active as u32);
    assert_eq!(tc_client.get_owner(&0), client.address.clone());
    assert_eq!(token_client.balance(&borrower.clone()), 10000000000000);
    assert_eq!(token_client.balance(&creditor.clone()), 0);

    // it should not be possible to cancel a loan offer once it has been accepted
    let res = client.try_cancel_loan_offer(&loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
}

#[test]
fn test_payoff_loan() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.add_whitelisted_tc(&tc_client.address);
    let loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);

    // it should not be possible to pay off the loan before the offer is accepted
    let res = client.try_payoff_loan(&loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
    client.accept_loan_offer(&borrower.clone(), &loan_id);

    client.payoff_loan(&loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Paid as u32);
    assert_eq!(token_client.balance(&borrower.clone()), 0);
    assert_eq!(token_client.balance(&client.address), 10000000000000);
}

#[test]
fn test_payoff_loan_with_interest() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.add_whitelisted_tc(&tc_client.address);
    client.set_fee_percent(&2);
    assert_eq!(client.get_pool_fee(), 2);
    let loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);
    assert_eq!(client.get_loan_fee(&loan_id), 2);
    client.accept_loan_offer(&borrower.clone(), &loan_id);

    token_admin_client.mint(&borrower.clone(), &200000000000);
    assert_eq!(token_client.balance(&borrower.clone()), 10200000000000);
    assert_eq!(client.get_payoff_amount(&loan_id), 10200000000000);
    client.payoff_loan(&loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Paid as u32);
    assert_eq!(token_client.balance(&borrower.clone()), 0);
    assert_eq!(token_client.balance(&client.address), 10200000000000);
}

#[test]
fn test_close_loan() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    token_admin_client.mint(&borrower.clone(), &10500000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<BytesN<32>>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.add_whitelisted_tc(&tc_client.address);
    client.set_fee_percent(&5);
    let loan_id = client.create_loan_offer(&creditor.clone(), &tc_client.address, &0);

    e.budget().reset_default();
    // it should not be possible to close the loan before the offer is accepted
    let res = client.try_close_loan(&loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
    client.accept_loan_offer(&borrower.clone(), &loan_id);

    // it should also not be possible to close the loan before the offer is paid off
    let res = client.try_close_loan(&loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );

    client.payoff_loan(&loan_id);
    client.close_loan(&loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Closed as u32);
    assert_eq!(token_client.balance(&creditor.clone()), 10500000000000);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
}
