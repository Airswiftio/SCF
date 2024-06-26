#![cfg(test)]
use crate::contract::{TokenizedCertificate, TokenizedCertificateClient};

use crate::errors::Error as ContractError;
use crate::storage_types::SplitRequest;
use crate::test_util::{set_ledger_timestamp, setup_test_token};
use soroban_sdk::{
    testutils::Address as _, token::Client as TokenClient, token::StellarAssetClient, vec, Address,
    Env, Error, String,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenizedCertificate);
    let client = TokenizedCertificateClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let total_amount: u32 = 1000000;
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    client.initialize(&admin, &buyer, &total_amount, &end_time);
    assert_eq!(admin, client.admin());
}

#[test]
fn test_initialize_invalid_end_time() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenizedCertificate);
    let client = TokenizedCertificateClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let total_amount: u32 = 1000000;
    let timestamp = 1672531200; // 2023-01-01 00:00:00 UTC+0
    set_ledger_timestamp(&env, timestamp);

    let end_time = timestamp - 86400;
    let res = client.try_initialize(&admin, &buyer, &total_amount, &end_time);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotPermitted as u32
        )))
    );
}

#[test]
fn test_mint_original() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));
    assert_eq!(1000000, client.amount(&0));
    assert_eq!(0, client.parent(&0));
    assert_eq!(false, client.is_disabled(&0));
    assert_eq!(vec![&env, String::from_str(&env, "a")], client.vc(&0));
}

#[test]
fn test_mint_original_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));

    let res = client.try_mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotEmpty as u32
        )))
    );
}

#[test]
fn test_add_vc() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(client.vc(&0), vec![&env, String::from_str(&env, "a")]);

    // add vc successfully
    client.add_vc(&0, &String::from_str(&env, "b"));
    assert_eq!(
        client.vc(&0),
        vec![
            &env,
            String::from_str(&env, "a"),
            String::from_str(&env, "b")
        ]
    );

    // attempt to add vc with string exceeding length limit of 2048: call should fail
    let bytes_data: [u8; 2049] = [b'a'; 2049];
    let long_vc = String::from_bytes(&env, &bytes_data);
    let res = client.try_add_vc(&0, &long_vc);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::VCStringTooLong as u32
        )))
    );

    // add 8 more vcs, resulting in a total of 10 vcs
    for _ in 0..8 {
        client.add_vc(&0, &String::from_str(&env, "i"));
    }
    assert_eq!(client.vc(&0).len(), 10);

    // attempt to add vc when vc limit is reached: call should fail
    let res = client.try_add_vc(&0, &String::from_str(&env, "n"));
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::VCListCapacityReached as u32
        )))
    );
}

#[test]
fn test_split() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
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

    client.add_vc(&1, &String::from_str(&env, "b"));
    client.add_vc(&3, &String::from_str(&env, "c"));
    client.add_vc(&3, &String::from_str(&env, "d"));

    assert_eq!(300000, client.amount(&1));
    assert_eq!(client.address, client.owner(&1));
    assert_eq!(0, client.parent(&1));
    assert_eq!(vec![&env, String::from_str(&env, "b")], client.vc(&1));

    assert_eq!(500000, client.amount(&2));
    assert_eq!(client.address, client.owner(&2));
    assert_eq!(0, client.parent(&2));
    assert_eq!(vec![&env], client.vc(&2));

    assert_eq!(200000, client.amount(&3));
    assert_eq!(to, client.owner(&3));
    assert_eq!(0, client.parent(&3));
    assert_eq!(
        vec![
            &env,
            String::from_str(&env, "c"),
            String::from_str(&env, "d")
        ],
        client.vc(&3)
    );

    assert_eq!(true, client.is_disabled(&0));
}

#[test]
fn test_split_nested() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
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
fn test_split_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
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

    let res = client.try_split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotPermitted as u32
        )))
    );
}

#[test]
fn test_split_exceed_total() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(1000000, client.amount(&0));

    let res = client.try_split(
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
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::AmountTooMuch as u32
        )))
    );
}

#[test]
fn test_split_empty() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    let res = client.try_split(&0, &vec![&env]);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidArgs as u32
        )))
    );
}

#[test]
fn test_split_minimum_amount() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(1000000, client.amount(&0));

    let res = client.try_split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 99999,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::SplitAmountTooLow as u32
        )))
    );
}

#[test]
fn test_split_nested_depth_limit() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(1000000, client.amount(&0));

    // first 5 splits should succeed, 6th split should fail
    for i in 0..6 {
        let parent_id = i * 2;
        let res = client.try_split(
            &parent_id,
            &vec![
                &env,
                SplitRequest {
                    amount: 100000,
                    to: to.clone(),
                },
            ],
        );
        if i < 5 {
            assert!(res.is_ok());
        } else {
            assert_eq!(
                res,
                Err(Ok(Error::from_contract_error(
                    ContractError::SplitLimitReached as u32
                )))
            );
        }
    }
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let acc1 = Address::generate(&env);
    let acc2 = Address::generate(&env);
    client.mint_original(&acc1, &String::from_str(&env, "a"));
    assert_eq!(acc1, client.owner(&0));

    client.transfer(&acc1, &acc2, &0);
    assert_eq!(acc2, client.owner(&0));
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    client.mint_original(&admin, &String::from_str(&env, "a"));
    let res = client.try_owner(&0);
    assert_eq!(res.is_ok(), true);

    client.burn(&0);
    let res2 = client.try_owner(&0);
    assert_eq!(res2.is_ok(), false);
}

#[test]
fn test_pay_off() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    // setup fake external token
    let ext_token_addr = &env.register_stellar_asset_contract(admin.clone());
    let ext_admin = StellarAssetClient::new(&env, ext_token_addr);
    ext_admin.mint(&buyer, &10000000000000);

    client.set_external_token_provider(&ext_token_addr, &7);
    assert_eq!(client.check_paid(), false);

    client.pay_off(&buyer);
    assert_eq!(client.check_paid(), true);
}

#[test]
fn test_check_expired() {
    let env = Env::default();
    set_ledger_timestamp(&env, 1640995200); // 2022-01-01 00:00:00 UTC+0
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    assert_eq!(client.check_expired(), false);

    set_ledger_timestamp(&env, 1672617600); // 2023-01-02 00:00:00 UTC +0
    assert_eq!(client.check_expired(), true);
}

#[test]
fn test_expire_auto_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    let to2 = Address::generate(&env);
    let to3 = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to2.clone(),
            },
            SplitRequest {
                amount: 100000,
                to: to2.clone(),
            },
        ],
    );
    client.sign_off(&1);
    client.split(
        &1,
        &vec![
            &env,
            SplitRequest {
                amount: 200000,
                to: to3.clone(),
            },
            SplitRequest {
                amount: 150000,
                to: to3.clone(),
            },
        ],
    );
    client.sign_off(&4);
    assert_eq!(to2, client.owner(&1));
    assert_eq!(client.address, client.owner(&2));
    assert_eq!(to, client.owner(&3));
    assert_eq!(to3, client.owner(&4));
    assert_eq!(client.address, client.owner(&5));
    assert_eq!(to2, client.owner(&6));

    set_ledger_timestamp(&env, 1672617600); // 2023-01-02 00:00:00 UTC +0
    assert_eq!(client.check_expired(), true);
    assert_eq!(to, client.owner(&2));
    assert_eq!(to2, client.owner(&5));
}

#[test]
fn test_redeem() {
    // setup env with specific timestamp
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    // setup fake external token and pay the contract
    let ext_token_addr = &env.register_stellar_asset_contract(admin.clone());
    let ext_admin = StellarAssetClient::new(&env, ext_token_addr);
    ext_admin.mint(&buyer, &10000000000000);
    let ext_client = TokenClient::new(&env, ext_token_addr);
    ext_client.mock_all_auths_allowing_non_root_auth();

    let supplier = Address::generate(&env);
    client.mint_original(&supplier, &String::from_str(&env, "a"));
    assert_eq!(supplier, client.owner(&0));

    // setup preconditions, and redeem should fail before all preconditions are met
    client.set_external_token_provider(&ext_token_addr, &7);
    assert_eq!(client.try_redeem(&0).is_err(), true);
    client.check_paid();
    assert_eq!(client.try_redeem(&0).is_err(), true);
    set_ledger_timestamp(&env, 1672617600); // 2023-01-02 00:00:00 UTC +0
    client.check_expired();

    assert_eq!(ext_client.balance(&supplier), 0);

    client.pay_off(&buyer);
    client.redeem(&0);

    // check balance was transferred
    assert_eq!(ext_client.balance(&supplier), 10000000000000);

    // check TC was burned
    assert_eq!(client.try_owner(&0).is_err(), true)
}

#[test]
fn test_sign_off() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
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
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    assert_eq!(vec![&env], client.get_all_owned(&to));

    client.mint_original(&to, &String::from_str(&env, "a"));

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
