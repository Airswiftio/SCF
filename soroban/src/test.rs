#![cfg(test)]
use crate::contract::{NonFungibleToken, NonFungibleTokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, symbol_short};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NonFungibleToken);
    let client = NonFungibleTokenClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let name = String::from_slice(&env, "testname");
    let symbol = symbol_short!("TEST");

    client.initialize(&admin, &name, &symbol);
    assert_eq!(admin, client.admin());
    assert_eq!(name, client.name());
    assert_eq!(symbol, client.symbol());
}

// TODO: write more tests