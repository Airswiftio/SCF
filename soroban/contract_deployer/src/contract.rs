//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::admin::{has_administrator, write_administrator};
use crate::errors::Error;
use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, BytesN, Env, IntoVal, Symbol, Val, Vec,
};

pub trait DeployerTrait {
    fn initialize(e: Env, admin: Address);
    fn deploy_contract(
        e: Env,
        deployer: Address,
        token_wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        init_fn_list: Vec<Symbol>,
        init_args_list: Vec<Vec<Val>>,
    ) -> Address;
}

#[contract]
pub struct Deployer;

#[contractimpl]
impl DeployerTrait for Deployer {
    fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic_with_error!(&e, Error::AlreadyInitialized);
        }
        write_administrator(&e, &admin);
    }

    fn deploy_contract(
        e: Env,
        deployer: Address,
        token_wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        init_fn_list: Vec<Symbol>,
        init_args_list: Vec<Vec<Val>>,
    ) -> Address {
        if deployer != e.current_contract_address() {
            deployer.require_auth();
        }
        if init_fn_list.len() != init_args_list.len() {
            panic_with_error!(&e, Error::ArgumentLengthMismatch);
        }

        let deployed_address = e
            .deployer()
            .with_address(deployer, salt)
            .deploy(token_wasm_hash);

        for i in 0..init_fn_list.len() {
            let _res: Val = e.invoke_contract(
                &deployed_address,
                &init_fn_list.get(i).unwrap(),
                init_args_list.get(i).unwrap(),
            );
        }

        deployed_address
    }
}
