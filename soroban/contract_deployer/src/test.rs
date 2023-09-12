#![cfg(test)]
extern crate std;



use soroban_sdk::{

    Address, BytesN, Env, IntoVal, log
};



fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}


#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let wasm_hash = install_token_wasm(&e);

    log!(&e, "Hello {}", wasm_hash);

    
}
