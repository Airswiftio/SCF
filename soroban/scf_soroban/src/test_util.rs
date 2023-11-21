#![cfg(any(test, feature = "testutils"))]

use crate::contract::{NonFungibleToken, NonFungibleTokenClient};
use soroban_sdk::{Address, Env};

pub fn setup_test_token<'a>(
    env: &Env,
    admin: &Address,
    buyer: &Address,
) -> NonFungibleTokenClient<'a> {
    let contract_id = env.register_contract(None, NonFungibleToken);
    let client = NonFungibleTokenClient::new(env, &contract_id);

    let invoice_num = 12345678;
    let po_num = 1;
    let total_amount: u32 = 1000000;
    let start_time = 1640995200; // 2022-01-01 00:00:00 UTC+0
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    client.initialize(
        admin,
        &invoice_num,
        &po_num,
        buyer,
        &total_amount,
        &start_time,
        &end_time,
    );
    client
}
