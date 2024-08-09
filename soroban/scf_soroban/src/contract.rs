use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::balance::{increment_supply, read_supply};
use crate::errors::Error;
use crate::event;
use crate::interface::TokenizedCertificateTrait;
use crate::loan::{
    has_loan_contract, read_loan_contract, read_loan_status, write_loan_contract, write_loan_status,
};
use crate::metadata::{read_external_token, write_external_token};
use crate::order_info::{read_order_info, write_order_info};
use crate::order_state::{read_paid, update_and_read_expired, write_paid};
use crate::owner::{
    add_vc, check_owner, read_all_owned, read_owner, read_recipient, read_vc, write_owner,
    write_recipient, write_vc,
};
use crate::storage_types::{SplitRequest, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use crate::sub_tc::{read_sub_tc, read_sub_tc_disabled, write_sub_tc, write_sub_tc_disabled};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, vec, Address, Env, String, Vec,
};

#[contract]
pub struct TokenizedCertificate;

#[contractimpl]
impl TokenizedCertificateTrait for TokenizedCertificate {
    fn initialize(
        e: Env,
        admin: Address,
        buyer_address: Address,
        total_amount: u32,
        end_time: u64,
    ) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        if end_time <= e.ledger().timestamp() {
            panic_with_error!(&e, Error::NotPermitted);
        }
        write_administrator(&e, &admin);
        //write_name(&e, &name);
        //write_symbol(&e, &symbol);
        write_order_info(&e, buyer_address, total_amount, end_time);
    }

    fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_administrator(&env)
    }

    fn set_admin(env: Env, new_admin: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    fn set_loan_contract(env: Env, contract_addr: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        if has_loan_contract(&env) {
            panic_with_error!(&env, Error::NotEmpty);
        }
        write_loan_contract(&env, &contract_addr);
        event::set_loan(&env, contract_addr);
    }

    fn set_loan_status(env: Env, id: i128, status: u32) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let loan_contract = read_loan_contract(&env);
        loan_contract.require_auth();

        // check id is a token that was already minted
        if read_supply(&env) <= id {
            panic_with_error!(&env, Error::NotFound);
        }
        // check token is still active
        if read_sub_tc_disabled(&env, id) {
            panic_with_error!(&env, Error::NotPermitted);
        }
        write_loan_status(&env, id, status);
        event::loan(&env, id, status);
    }

    fn amount(env: Env, id: i128) -> u32 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let sub_tc = read_sub_tc(&env, id);
        sub_tc.amount
    }

    fn parent(env: Env, id: i128) -> i128 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let sub_tc = read_sub_tc(&env, id);
        sub_tc.parent
    }

    fn owner(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        read_owner(&env, id)
    }

    fn vc(env: Env, id: i128) -> Vec<String> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_vc(&env, id)
    }

    fn loan_status(env: Env, id: i128) -> u32 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_loan_status(&env, id)
    }

    fn get_all_owned(env: Env, address: Address) -> Vec<i128> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        read_all_owned(&env, address)
    }

    fn is_disabled(env: Env, id: i128) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_sub_tc_disabled(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        check_owner(&env, &from, id);
        from.require_auth();
        write_owner(&env, id, Some(to.clone()));
        event::transfer(&env, from, to, id);
    }

    fn mint_original(env: Env, to: Address, vc: String) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        let id = read_supply(&env);
        if id != 0 {
            panic_with_error!(&env, Error::NotEmpty);
        }
        let amount = read_order_info(&env).total_amount;
        write_owner(&env, id, Some(to.clone()));
        write_sub_tc(&env, id, id, 0, amount);
        add_vc(&env, id, vc);
        write_sub_tc_disabled(&env, id, false);
        write_loan_status(&env, id, 0);
        increment_supply(&env);

        event::mint(&env, to, id)
    }

    fn burn(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        update_and_read_expired(&env);
        let from = read_owner(&env, id);
        write_owner(&env, id, None);

        event::burn(&env, from, id);
    }

    fn split(env: Env, id: i128, splits: Vec<SplitRequest>) -> Vec<i128> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if read_sub_tc_disabled(&env, id) {
            // if the TC is disabled, it has already been split
            panic_with_error!(&env, Error::NotPermitted);
        }
        if splits.len() == 0 {
            panic_with_error!(&env, Error::InvalidArgs);
        }
        if update_and_read_expired(&env) {
            panic_with_error!(&env, Error::NotPermitted);
        }
        if read_loan_status(&env, id) != 0 {
            panic_with_error!(&env, Error::NotPermitted);
        }
        let owner = read_owner(&env, id);
        owner.require_auth();
        let contract_addr = env.current_contract_address();

        let parent = read_sub_tc(&env, id);
        if parent.depth >= 5 {
            panic_with_error!(&env, Error::SplitLimitReached);
        }
        let mut sum = 0;
        let root_total = read_order_info(&env).total_amount;
        for req in splits.clone() {
            // each split must be at least 10% of the root total_amount
            if req.amount * 10 < root_total {
                panic_with_error!(&env, Error::SplitAmountTooLow);
            }
            sum += req.amount;
        }
        if sum > parent.amount {
            panic_with_error!(&env, Error::AmountTooMuch);
        }

        let mut remaining = parent.amount;
        let mut new_ids = Vec::new(&env);
        for req in splits.clone() {
            let new_id = read_supply(&env);
            write_sub_tc(&env, new_id, id, parent.depth + 1, req.amount);
            write_sub_tc_disabled(&env, new_id, false);
            write_loan_status(&env, new_id, 0);
            write_recipient(&env, new_id, &req.to);
            write_owner(&env, new_id, Some(contract_addr.clone()));
            write_vc(&env, new_id, vec![&env]);
            increment_supply(&env);
            new_ids.push_back(new_id);
            remaining -= req.amount;
        }

        // if root amount > 0, create another sub tc to represent the remaining amount belonging to original owner
        if remaining > 0 {
            let new_id = read_supply(&env);
            write_sub_tc(&env, new_id, id, parent.depth + 1, remaining);
            write_sub_tc_disabled(&env, new_id, false);
            write_loan_status(&env, new_id, 0);
            write_owner(&env, new_id, Some(owner.clone()));
            write_vc(&env, new_id, vec![&env]);
            increment_supply(&env);
            new_ids.push_back(new_id);
        }

        // disable the original TC
        write_sub_tc_disabled(&env, id, true);

        event::split(&env, owner, id, new_ids.clone());
        new_ids
    }

    fn redeem(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if !update_and_read_expired(&env)
            || !read_paid(&env)
            || read_sub_tc_disabled(&env, id)
            || read_loan_status(&env, id) == 1
        {
            panic_with_error!(&env, Error::NotPermitted);
        }

        let owner = read_owner(&env, id);
        owner.require_auth();

        // send funds to owner address
        let sub_tc = read_sub_tc(&env, id);
        let ext_token = read_external_token(&env);
        let client = token::Client::new(&env, &ext_token.contract_addr);
        let amount = i128::from(sub_tc.amount) * 10i128.pow(ext_token.decimals);
        client.transfer(&env.current_contract_address(), &owner, &amount);

        // burn the token
        write_owner(&env, id, None);

        event::redeem(&env, owner, id);
    }

    fn set_external_token_provider(env: Env, contract_addr: Address, decimals: u32) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_external_token(&env, contract_addr, decimals);
    }

    fn check_paid(env: Env) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_paid(&env)
    }

    fn check_expired(env: Env) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env)
    }

    fn recipient(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_recipient(&env, id)
    }

    fn sign_off(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let expired = update_and_read_expired(&env);
        let owner = read_owner(&env, id);
        if owner != env.current_contract_address() || read_sub_tc_disabled(&env, id) || expired {
            panic_with_error!(&env, Error::NotPermitted);
        }

        let recipient = read_recipient(&env, id);
        recipient.require_auth();

        write_owner(&env, id, Some(recipient.clone()));

        event::transfer(&env, owner, recipient, id);
    }

    fn pay_off(env: Env, from: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let paid = read_paid(&env);
        if paid {
            panic_with_error!(&env, Error::NotEmpty);
        }
        let ext_token = read_external_token(&env);
        let client = token::Client::new(&env, &ext_token.contract_addr);
        let order_info = read_order_info(&env);
        let amount = i128::from(order_info.total_amount) * 10i128.pow(ext_token.decimals);

        if from != order_info.buyer_address {
            panic_with_error!(&env, Error::NotAuthorized);
        }
        from.require_auth();
        client.transfer(&from, &env.current_contract_address(), &i128::from(amount));
        write_paid(&env, true);
    }

    fn add_vc(env: Env, id: i128, vc: String) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        add_vc(&env, id, vc);
    }
}
