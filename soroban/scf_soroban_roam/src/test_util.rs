#![cfg(any(test, feature = "testutils"))]

use crate::contract::{TokenizedCertificate, TokenizedCertificateArgs, TokenizedCertificateClient};
use soroban_sdk::{testutils::Ledger as _, Address, Env};

pub fn setup_test_token<'a>(
    env: &Env,
    admin: &Address,
    buyer: &Address,
) -> TokenizedCertificateClient<'a> {
    let total_amount: u32 = 1000000;
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    let contract_id = env.register(
        TokenizedCertificate,
        TokenizedCertificateArgs::__constructor(admin, buyer, &total_amount, &end_time),
    );
    let client = TokenizedCertificateClient::new(env, &contract_id);

    client
}

pub fn set_ledger_timestamp(e: &Env, timestamp: u64) {
    e.ledger().with_mut(|li| li.timestamp = timestamp);
}
