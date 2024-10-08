use crate::{
    admin::{has_admin, read_admin, write_admin},
    errors::Error,
    ext_token::{read_ext_token, write_ext_token},
    interface::LiquidityPoolTrait,
    loan::{
        increment_supply, is_whitelisted, read_fee_percent, read_loan, read_supply, read_whitelist,
        write_fee_percent, write_loan, write_whitelist, Loan, LoanStatus,
    },
    storage_types::{TokenInfo, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{contract, contractimpl, panic_with_error, token, Address, Env, Vec};

mod tc_contract {
    soroban_sdk::contractimport!(
        file = "../argentina_pledge/target/wasm32-unknown-unknown/release/argentina_pledge.wasm"
    );
}

#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(e: Env, admin: Address, ext_token_address: Address, fee_percent: u32) {
        if has_admin(&e) {
            panic!("already initialized")
        }
        let ext_token_decimals = token::Client::new(&e, &ext_token_address).decimals();

        write_admin(&e, &admin);
        if ext_token_decimals > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        write_ext_token(
            &e,
            TokenInfo {
                address: ext_token_address,
                decimals: ext_token_decimals,
            },
        );
        write_fee_percent(&e, fee_percent);
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_admin(&e, &new_admin);
    }

    fn set_fee_percent(e: Env, new_fee_percentage: u32) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_fee_percent(&e, new_fee_percentage);
    }

    fn add_whitelisted_tc(e: Env, tc_address: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let mut tc_whitelist = read_whitelist(&e);
        if tc_whitelist.contains_key(tc_address.clone()) {
            return;
        }
        tc_whitelist.set(tc_address.clone(), ());
        write_whitelist(&e, tc_whitelist);
    }

    fn remove_whitelisted_tc(e: Env, tc_address: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let mut tc_whitelist = read_whitelist(&e);
        if !tc_whitelist.contains_key(tc_address.clone()) {
            return;
        }
        tc_whitelist.remove(tc_address.clone());
        write_whitelist(&e, tc_whitelist);
    }

    fn get_whitelisted_tcs(e: Env) -> Vec<Address> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let whitelist = read_whitelist(&e);
        whitelist.keys()
    }

    fn create_loan_offer(e: Env, from: Address, tc_address: Address, tc_id: u64) -> u64 {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        if !is_whitelisted(&e, tc_address.clone()) {
            panic_with_error!(&e, Error::TCNotWhitelisted);
        }
        let offer_id = read_supply(&e);
        let tc_amount = i128::from(tc_contract::Client::new(&e, &tc_address).get_amount(&tc_id));
        // lock in funds from caller (potential creditor)
        transfer_scaled(&e, from.clone(), e.current_contract_address(), tc_amount, 0);
        let request = Loan {
            borrower: from.clone(),
            creditor: from.clone(),
            amount: i128::from(tc_amount),
            tc_address,
            tc_id,
            fee_percent: read_fee_percent(&e),
            status: LoanStatus::Pending,
        };

        write_loan(&e, offer_id, request);
        increment_supply(&e);
        return offer_id;
    }

    fn cancel_loan_offer(e: Env, offer_id: u64) {
        let mut loan = read_loan(&e, offer_id);
        loan.creditor.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        if loan.status != LoanStatus::Pending {
            panic_with_error!(&e, Error::InvalidStatus);
        }

        // return funds from smart contract to creditor
        transfer_scaled(
            &e,
            e.current_contract_address(),
            loan.creditor.clone(),
            loan.amount,
            0,
        );

        loan.status = LoanStatus::Closed;
        write_loan(&e, offer_id, loan);
    }

    fn accept_loan_offer(e: Env, from: Address, offer_id: u64) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let mut loan = read_loan(&e, offer_id);
        if loan.status != LoanStatus::Pending {
            panic_with_error!(&e, Error::InvalidStatus);
        }

        // transfer the TC from caller (borrower) to smart contract
        tc_contract::Client::new(&e, &loan.tc_address).transfer(
            &from.clone(),
            &e.current_contract_address(),
            &loan.tc_id,
        );

        // transfer liquidity tokens from smart contract to caller (borrower)
        transfer_scaled(
            &e,
            e.current_contract_address(),
            from.clone(),
            loan.amount,
            0,
        );

        // update loan info
        loan.borrower = from;
        loan.status = LoanStatus::Active;
        write_loan(&e, offer_id, loan);
    }

    fn payoff_loan(e: Env, offer_id: u64) {
        let mut loan = read_loan(&e, offer_id);
        if loan.status != LoanStatus::Active {
            panic_with_error!(&e, Error::InvalidStatus);
        }
        loan.borrower.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // transfer liquidity tokens from caller (borrower) to creditor
        // pool fee_percent is the additional percentage fee needed to pay off the loan.
        transfer_scaled(
            &e,
            loan.borrower.clone(),
            loan.creditor.clone(),
            loan.amount,
            loan.fee_percent,
        );

        // transfer the TC from smart contract to borrower
        tc_contract::Client::new(&e, &loan.tc_address).transfer(
            &e.current_contract_address(),
            &loan.borrower,
            &loan.tc_id,
        );

        // update loan info
        loan.status = LoanStatus::Closed;
        write_loan(&e, offer_id, loan);
    }

    fn default_loan(e: Env, offer_id: u64) {
        let admin = read_admin(&e);
        admin.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let mut loan = read_loan(&e, offer_id);
        if loan.status != LoanStatus::Active {
            panic_with_error!(&e, Error::InvalidStatus);
        }

        // transfer the TC from smart contract to creditor
        tc_contract::Client::new(&e, &loan.tc_address).transfer(
            &e.current_contract_address(),
            &loan.creditor,
            &loan.tc_id,
        );

        // update loan info
        loan.status = LoanStatus::Defaulted;
        write_loan(&e, offer_id, loan);
    }

    fn get_loan_fee(e: Env, offer_id: u64) -> u32 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.fee_percent
    }

    fn get_pool_fee(e: Env) -> u32 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_fee_percent(&e)
    }

    fn get_loan_tc(e: Env, offer_id: u64) -> (Address, u64) {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        (loan.tc_address, loan.tc_id)
    }

    fn get_loan_borrower(e: Env, offer_id: u64) -> Address {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.borrower
    }

    fn get_loan_creditor(e: Env, offer_id: u64) -> Address {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.creditor
    }

    fn get_ext_token(e: Env) -> (Address, u32) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let ext_token = read_ext_token(&e);
        (ext_token.address, ext_token.decimals)
    }

    fn get_payoff_amount(e: Env, offer_id: u64) -> i128 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let scaled_amount = calculate_scaled_amount_with_interest(
            loan.amount,
            read_ext_token(&e).decimals,
            loan.fee_percent,
        );
        match scaled_amount {
            Some(scaled_amount) => scaled_amount,
            None => panic_with_error!(&e, Error::IntegerOverflow),
        }
    }

    fn get_loan_amount(e: Env, offer_id: u64) -> i128 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.amount
    }

    fn get_loan_status(e: Env, offer_id: u64) -> u32 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.status as u32
    }
}

fn transfer_scaled(e: &Env, from: Address, to: Address, amount: i128, added_percentage: u32) {
    let ext_token = read_ext_token(&e);
    let scaled_amount =
        calculate_scaled_amount_with_interest(amount, ext_token.decimals, added_percentage);
    match scaled_amount {
        Some(scaled_amount) => {
            token::Client::new(&e, &ext_token.address).transfer(&from, &to, &scaled_amount);
        }
        None => panic_with_error!(&e, Error::IntegerOverflow),
    }
}

fn calculate_scaled_amount_with_interest(
    amount: i128,
    decimals: u32,
    added_percentage: u32,
) -> Option<i128> {
    if added_percentage == 0 {
        return amount.checked_mul(10i128.pow(decimals));
    }
    amount
        .checked_mul(10i128.pow(decimals))?
        .checked_mul(100 + i128::from(added_percentage))?
        .checked_div(100)
}
