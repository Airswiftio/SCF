#![cfg(any(test, feature = "testutils"))]

use crate::contract::{NonFungibleToken, NonFungibleTokenClient};
use soroban_sdk::{Address, Env, String};

pub fn setup_test_token<'a>(env: &Env, admin: &Address) -> NonFungibleTokenClient<'a> {
    let contract_id = env.register_contract(None, NonFungibleToken);
    let client = NonFungibleTokenClient::new(env, &contract_id);

    let invoice_num = 12345678;
    let po_num = 1;
    let total_amount: u32 = 1000000;
    let checksum = String::from_slice(env, "1f1e33");
    let supplier_name = String::from_slice(env, "L1 Supplier");
    let buyer_name = String::from_slice(env, "Buyer Company");
    let start_time = 1640995200; // 2022-01-01 00:00:00 UTC+0
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    client.initialize(
        admin,
        &invoice_num,
        &po_num,
        &total_amount,
        &checksum,
        &supplier_name,
        &buyer_name,
        &start_time,
        &end_time,
    );
    client
}
