//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::admin::{has_administrator, read_administrator, write_administrator};
use soroban_sdk::{contract, contractimpl, symbol_short, Address,  Env,  Bytes, BytesN, String, IntoVal, Val, Vec};



pub trait DeployerTrait {
    fn initialize(e: Env, admin: Address);
    fn DeployContract(e: Env, token_wasm_hash: BytesN<32>,  salt: BytesN<32> )-> Address;

}


#[contract]
pub struct Deployer;

#[contractimpl]
impl DeployerTrait for Deployer {
    fn initialize(e: Env, admin: Address) {
        
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
    }


    fn DeployContract(e: Env, token_wasm_hash: BytesN<32>,  salt: BytesN<32> )-> Address {
        
        // let admin = read_administrator(&e);
        // admin.require_auth();

        let deployed_address = e
        .deployer()
        .with_address(e.current_contract_address(), salt)
        .deploy(token_wasm_hash);

        // let init_fn = symbol_short!("init");


        // let res: Val = e.invoke_contract(&deployed_address, &init_fn, init_args);
        
        (deployed_address)
    }

}
