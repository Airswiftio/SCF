use crate::{
    admin::{has_admin, read_admin, write_admin},
    errors::Error,
    ext_token::{read_ext_token, write_ext_token},
    interface::LiquidityPoolTrait,
    loan::{has_loan, write_loan, write_rate_percent, Loan, LoanStatus},
    pool_token::{create_contract, read_pool_token, write_pool_token},
    storage_types::{TokenInfo, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, vec, Address, BytesN, Env, IntoVal, Symbol,
    Val,
};

#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(
        e: Env,
        admin: Address,
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,
        rate_percent: u32,
    ) {
        if has_admin(&e) {
            panic!("already initialized")
        }
        write_admin(&e, &admin);
        if ext_token_decimals > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        // deploy and initialize the token contract
        let pool_token_contract = create_contract(&e, token_wasm_hash);
        e.invoke_contract::<Val>(
            &pool_token_contract,
            &"initialize".into_val(&e),
            vec![
                &e,
                e.current_contract_address().into_val(&e),
                7u32.into_val(&e),
                "Argentina Pool Token".into_val(&e),
                "APT".into_val(&e),
            ],
        );

        write_pool_token(
            &e,
            TokenInfo {
                address: pool_token_contract,
                decimals: 7,
            },
        );
        write_ext_token(
            &e,
            TokenInfo {
                address: ext_token_address,
                decimals: ext_token_decimals,
            },
        );
        write_rate_percent(&e, rate_percent);
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_admin(&e, &new_admin);
    }

    fn set_rate(e: Env, new_rate: u32) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_rate_percent(&e, new_rate);
    }

    fn deposit(e: Env, from: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Transfer USDC from "from" to the contract address
        let ext_token = read_ext_token(&e);
        let client = token::Client::new(&e, &ext_token.address);
        client.transfer(&from, &e.current_contract_address(), &amount);

        // Mint an equal number of liquidity tokens to "from"
        token::StellarAssetClient::new(&e, &read_pool_token(&e).address).mint(&from, &amount);
    }

    fn withdraw(e: Env, from: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Burn the specified number of liquidity tokens from "from"
        token::Client::new(&e, &read_pool_token(&e).address).burn(&from, &amount);

        // Transfer USDC from the contract address to "from"
        token::Client::new(&e, &read_ext_token(&e).address).transfer(
            &e.current_contract_address(),
            &from,
            &amount,
        );
    }

    fn create_loan_offer(e: Env, from: Address, offer_id: i128, tc_address: Address, tc_id: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        if has_loan(&e, offer_id) {
            panic_with_error!(&e, Error::NotEmpty);
        }
        let request = Loan {
            id: offer_id,
            borrower: from.clone(),
            creditor: from.clone(),
            amount: 0,
            tc_address,
            tc_id,
            status: LoanStatus::Pending,
        };

        write_loan(&e, request);
    }
}
