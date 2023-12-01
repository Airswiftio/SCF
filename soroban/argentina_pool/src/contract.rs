use crate::interface::LiquidityPoolTrait;
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(
        e: Env,
        admin: Address,
        ext_token_address: Address,
        ext_token_decimals: u32,
        rate: u32,
    ) {
    }
}
