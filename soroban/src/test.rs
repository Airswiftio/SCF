#![cfg(test)]
use crate::contract::{NonFungibleToken, NonFungibleTokenClient, __is_disabled};
use crate::errors::Error;
use crate::test_util::setup_test_token;
use soroban_sdk::Vec;
use soroban_sdk::{symbol_short, testutils::Address as _, vec, Address, Env, String};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NonFungibleToken);
    let client = NonFungibleTokenClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let invoice_num = String::from_slice(&env, "12345678");
    let po_num = String::from_slice(&env, "1");
    let total_amount: u32 = 1000000;
    let checksum = String::from_slice(&env, "1f1e33");
    let supplier_name = String::from_slice(&env, "L1 Supplier");
    let buyer_name = String::from_slice(&env, "Buyer Company");
    let start_date = String::from_slice(&env, "2023-08-01");
    let end_date = String::from_slice(&env, "2024-08-01");

    client.initialize(
        &admin,
        &invoice_num,
        &po_num,
        &total_amount,
        &checksum,
        &supplier_name,
        &buyer_name,
        &start_date,
        &end_date,
    );
    assert_eq!(admin, client.admin());
    // TODO: getters for other fields?
}

#[test]
fn test_mint_original() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(to, client.owner(&0));
    assert_eq!(1000000, client.amount(&0));
    assert_eq!(0, client.parent(&0));
    assert_eq!(false, client.is_disabled(&0));
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_mint_original_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(to, client.owner(&0));

    client.mint_original(&to); // should panic
}

#[test]
fn test_split() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(1000000, client.amount(&0));

    client.split(&0, &vec![&env, 300000_u32, 500000_u32]);

    assert_eq!(300000, client.amount(&1));
    assert_eq!(admin, client.owner(&1));
    assert_eq!(0, client.parent(&1));

    assert_eq!(500000, client.amount(&2));
    assert_eq!(admin, client.owner(&2));
    assert_eq!(0, client.parent(&2));

    assert_eq!(200000, client.amount(&3));
    assert_eq!(to, client.owner(&3));
    assert_eq!(0, client.parent(&3));

    assert_eq!(true, client.is_disabled(&0));
}

#[test]
fn test_split_nested() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(1000000, client.amount(&0));

    client.split(&0, &vec![&env, 800000_u32]);
    assert_eq!(800000, client.amount(&1));
    // remaining token id 2 is worth 200k and belongs to buyer

    client.split(&1, &vec![&env, 500000_u32]);
    assert_eq!(500000, client.amount(&3));
    assert_eq!(admin, client.owner(&3));
    assert_eq!(1, client.parent(&3));

    assert_eq!(300000, client.amount(&4));
    assert_eq!(admin, client.owner(&4));
    assert_eq!(1, client.parent(&4));
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_split_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    client.split(&0, &vec![&env, 500000_u32]);
    client.split(&0, &vec![&env, 500000_u32]);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_split_exceed() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(1000000, client.amount(&0));

    client.split(&0, &vec![&env, 500000_u32, 5000001_u32]);
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn test_split_empty() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    client.split(&0, &vec![&env]);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let acc1 = Address::random(&env);
    let acc2 = Address::random(&env);
    client.mint_original(&acc1);
    assert_eq!(acc1, client.owner(&0));

    client.transfer(&acc1, &acc2, &0);
    assert_eq!(acc2, client.owner(&0));
}
