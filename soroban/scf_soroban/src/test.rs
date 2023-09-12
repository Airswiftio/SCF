#![cfg(test)]
use crate::contract::{NonFungibleToken, NonFungibleTokenClient};

use crate::storage_types::SplitRequest;
use crate::test_util::setup_test_token;
use soroban_sdk::{
    testutils::Address as _, token::AdminClient as TokenAdminClient, token::Client as TokenClient,
    vec, Address, Env, String,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NonFungibleToken);
    let client = NonFungibleTokenClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let invoice_num = 12345678;
    let po_num = 1;
    let total_amount: u32 = 1000000;
    let checksum = String::from_slice(&env, "1f1e33");
    let supplier_name = String::from_slice(&env, "L1 Supplier");
    let buyer_name = String::from_slice(&env, "Buyer Company");
    let start_time = 1640995200; // 2022-01-01 00:00:00 UTC+0
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    client.initialize(
        &admin,
        &invoice_num,
        &po_num,
        &total_amount,
        &checksum,
        &supplier_name,
        &buyer_name,
        &start_time,
        &end_time,
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

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 300000,
                to: to.clone(),
            },
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );

    assert_eq!(300000, client.amount(&1));
    assert_eq!(client.address, client.owner(&1));
    assert_eq!(0, client.parent(&1));

    assert_eq!(500000, client.amount(&2));
    assert_eq!(client.address, client.owner(&2));
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

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 800000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(800000, client.amount(&1));
    // remaining token id 2 is worth 200k and belongs to buyer

    client.split(
        &1,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(500000, client.amount(&3));
    assert_eq!(client.address, client.owner(&3));
    assert_eq!(1, client.parent(&3));

    assert_eq!(300000, client.amount(&4));
    assert_eq!(client.address, client.owner(&4));
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
    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
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

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
            SplitRequest {
                amount: 500001,
                to: to.clone(),
            },
        ],
    );
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

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    client.mint_original(&admin);
    let res = client.try_owner(&0);
    assert_eq!(res.is_ok(), true);

    client.burn(&0);
    let res2 = client.try_owner(&0);
    assert_eq!(res2.is_ok(), false);
}

#[test]
fn test_check_paid() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);
    let user = Address::random(&env);

    // setup fake external token
    let ext_token_addr = &env.register_stellar_asset_contract(admin.clone());
    let ext_admin = TokenAdminClient::new(&env, ext_token_addr);
    ext_admin.mint(&user, &1000000);
    let ext_client = TokenClient::new(&env, ext_token_addr);
    ext_client.mock_all_auths();

    client.set_external_token_provider(&ext_token_addr);
    assert_eq!(client.check_paid(), false);

    ext_client.transfer(&user, &client.address, &500000);
    assert_eq!(client.check_paid(), false); // total amount is 1000000, only 500000 was paid

    ext_client.transfer(&user, &client.address, &500000);
    assert_eq!(client.check_paid(), true); // successfully paid the rest of the amount
}

#[test]
fn test_check_expired() {
    let mut snapshot = Env::default().to_snapshot();
    snapshot.timestamp = 1640995200; // 2022-01-01 00:00:00 UTC+0
    let env = Env::from_snapshot(snapshot.clone());
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    assert_eq!(client.check_expired(), false);

    let mut snapshot2 = Env::default().to_snapshot();
    snapshot2.timestamp = 1672617600; // 2023-01-02 00:00:00 UTC +0
    let env2 = Env::from_snapshot(snapshot2.clone());
    let client2 = setup_test_token(&env2, &admin);
    assert_eq!(client2.check_expired(), true);
}

#[test]
fn test_expire_auto_transfer() {
    let mut snapshot = Env::default().to_snapshot();
    snapshot.timestamp = 1672617600; // 2023-01-02 00:00:00 UTC +0
    let env = Env::from_snapshot(snapshot.clone());
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(to, client.owner(&0));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 600000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(client.address, client.owner(&1));

    assert_eq!(client.check_expired(), true);
    assert_eq!(to, client.owner(&1));
}

#[test]
fn test_redeem() {
    // setup env with specific timestamp
    let mut snapshot = Env::default().to_snapshot();
    snapshot.timestamp = 1672617600; // 2023-01-02 00:00:00 UTC +0
    let env = Env::from_snapshot(snapshot.clone());
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    // setup fake external token and pay the contract
    let buyer = Address::random(&env);
    let ext_token_addr = &env.register_stellar_asset_contract(admin.clone());
    let ext_admin = TokenAdminClient::new(&env, ext_token_addr);
    ext_admin.mint(&buyer, &1000000);
    let ext_client = TokenClient::new(&env, ext_token_addr);
    ext_client.mock_all_auths();
    ext_client.transfer(&buyer, &client.address, &1000000);

    let supplier = Address::random(&env);
    client.mint_original(&supplier);
    assert_eq!(supplier, client.owner(&0));

    // setup preconditions, and redeem should fail before all preconditions are met
    client.set_external_token_provider(&ext_token_addr);
    assert_eq!(client.try_redeem(&0).is_err(), true);
    client.check_paid();
    assert_eq!(client.try_redeem(&0).is_err(), true);
    client.check_expired();

    assert_eq!(ext_client.balance(&supplier), 0);
    client.redeem(&0);

    // check balance was transferred
    assert_eq!(ext_client.balance(&supplier), 1000000);

    // check NFT was burned
    assert_eq!(client.try_owner(&0).is_err(), true)
}

#[test]
fn test_sign_off() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    client.mint_original(&to);
    assert_eq!(to, client.owner(&0));

    let split_req = SplitRequest {
        amount: 600000,
        to: to.clone(),
    };
    client.split(&0, &vec![&env, split_req.clone()]);
    assert_eq!(client.address, client.owner(&1));
    assert_eq!(to, client.recipient(&1));
    client.sign_off(&1);
    assert_eq!(to, client.owner(&1));
}

#[test]
fn test_get_all_owned() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::random(&env);
    assert_eq!(vec![&env], client.get_all_owned(&to));

    client.mint_original(&to);

    assert_eq!(vec![&env, 0], client.get_all_owned(&to));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 100000,
                to: to.clone(),
            },
            SplitRequest {
                amount: 200000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(vec![&env, 3], client.get_all_owned(&to));

    client.sign_off(&1);
    client.sign_off(&2);
    assert_eq!(vec![&env, 1, 2, 3], client.get_all_owned(&to));
}
