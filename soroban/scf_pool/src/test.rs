#![cfg(test)]
extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, BytesN, Env, IntoVal,
};

// fn install_token_wasm(e: &Env) -> BytesN<32> {
//     soroban_sdk::contractimport!(
//         file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
//     );
//     e.deployer().upload_contract_wasm(WASM)
// }

// fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
//     token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
// }

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    // let wasm_hash = install_token_wasm(&e);
}
