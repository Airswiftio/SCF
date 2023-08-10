use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::approval::{read_approval, read_approval_all, write_approval, write_approval_all};
use crate::balance::{increment_supply, read_supply};
use crate::errors::Error;
use crate::event;
use crate::interface::NonFungibleTokenTrait;
use crate::metadata::{
    read_expired, read_external_token_provider, read_name, read_paid, read_symbol, read_token_uri,
    write_expired, write_external_token_provider, write_paid,
};
use crate::order_info::{read_end_time, read_total_amount, write_order_info};
use crate::owner::{check_owner, read_owner, write_owner};
use crate::storage_types::INSTANCE_BUMP_AMOUNT;
use crate::sub_nft::{read_sub_nft, read_sub_nft_disabled, write_sub_nft, write_sub_nft_disabled};
use soroban_sdk::{
    contract, contractimpl, log, panic_with_error, token, Address, Env, String, Symbol, Vec,
};

#[contract]
pub struct NonFungibleToken;

#[contractimpl]
impl NonFungibleTokenTrait for NonFungibleToken {
    fn initialize(
        e: Env,
        admin: Address,
        invoice_num: i128,
        po_num: i128,
        total_amount: u32,
        checksum: String,
        supplier_name: String,
        buyer_name: String,
        start_time: u64,
        end_time: u64,
    ) {
        if has_administrator(&e) {
            panic!("already initialized")
        }

        write_administrator(&e, &admin);
        //write_name(&e, &name);
        //write_symbol(&e, &symbol);
        write_order_info(
            &e,
            invoice_num,
            po_num,
            total_amount,
            checksum,
            supplier_name,
            buyer_name,
            start_time,
            end_time,
        );
    }

    fn admin(env: Env) -> Address {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_administrator(&env)
    }

    fn set_admin(env: Env, new_admin: Address) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    fn name(env: Env) -> String {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_name(&env)
    }

    fn symbol(env: Env) -> Symbol {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_symbol(&env)
    }

    fn token_uri(env: Env, id: i128) -> String {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_token_uri(&env, id)
    }

    fn appr(env: Env, owner: Address, operator: Address, id: i128) {
        owner.require_auth();
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &owner, id);

        write_approval(&env, id, Some(operator.clone()));
        event::approve(&env, operator, id);
    }

    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool) {
        owner.require_auth();
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        write_approval_all(&env, owner.clone(), operator.clone(), approved);
        event::approve_all(&env, operator, owner)
    }

    fn get_appr(env: Env, id: i128) -> Address {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_approval(&env, id)
    }

    fn is_appr(env: Env, owner: Address, operator: Address) -> bool {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_approval_all(&env, owner, operator)
    }

    fn amount(env: Env, id: i128) -> u32 {
        let sub_nft = read_sub_nft(&env, id);
        sub_nft.amount
    }

    fn parent(env: Env, id: i128) -> i128 {
        let sub_nft = read_sub_nft(&env, id);
        sub_nft.root
    }

    fn owner(env: Env, id: i128) -> Address {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_owner(&env, id)
    }

    fn is_disabled(env: Env, id: i128) -> bool {
        read_sub_nft_disabled(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, id: i128) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &from, id);
        from.require_auth();
        write_owner(&env, id, Some(to.clone()));
        event::transfer(&env, from, to, id);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &from, id);
        spender.require_auth();

        if read_approval_all(&env, from.clone(), spender.clone())
            || spender == read_approval(&env, id)
        {
            write_approval(&env, id, None);

            write_owner(&env, id, Some(to.clone()));

            event::transfer(&env, from, to, id);
        } else {
            panic_with_error!(&env, Error::NotAuthorized)
        }
    }

    fn mint_original(env: Env, to: Address) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        let id = read_supply(&env);
        if id != 0 {
            panic_with_error!(&env, Error::NotEmpty);
        }
        let amount = read_total_amount(&env);
        write_owner(&env, id, Some(to.clone()));
        write_sub_nft(&env, id, id, amount);
        write_sub_nft_disabled(&env, id, false);
        increment_supply(&env);

        event::mint(&env, to, id)
    }

    fn burn(env: Env, id: i128) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        let from = read_owner(&env, id);
        write_owner(&env, id, None);

        event::burn(&env, from, id);
    }

    fn split(env: Env, id: i128, amounts: Vec<u32>) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        if read_sub_nft_disabled(&env, id) {
            // if the NFT is disabled, it has already been split
            panic_with_error!(&env, Error::NotPermitted);
        }
        if amounts.len() == 0 {
            panic_with_error!(&env, Error::InvalidArgs);
        }
        if read_expired(&env) {
            panic_with_error!(&env, Error::NotPermitted);
        }
        let owner = read_owner(&env, id);
        owner.require_auth();
        let contract_addr = env.current_contract_address();

        let root = read_sub_nft(&env, id);
        if amounts.iter().sum::<u32>() > root.amount {
            panic_with_error!(&env, Error::AmountTooMuch);
        }

        let mut remaining = root.amount;
        let mut new_ids = Vec::new(&env);
        for amount in amounts {
            let new_id = read_supply(&env);
            write_sub_nft(&env, new_id, id, amount);
            write_sub_nft_disabled(&env, new_id, false);
            write_owner(&env, new_id, Some(contract_addr.clone()));
            increment_supply(&env);
            new_ids.push_back(new_id);
            remaining -= amount;
        }

        // if root amount > 0, create another sub nft to represent the remaining amount belonging to original owner
        if remaining > 0 {
            let new_id = read_supply(&env);
            write_sub_nft(&env, new_id, id, remaining);
            write_sub_nft_disabled(&env, new_id, false);
            write_owner(&env, new_id, Some(owner.clone()));
            increment_supply(&env);
            new_ids.push_back(new_id);
        }

        // disable the original NFT
        write_sub_nft_disabled(&env, id, true);

        event::split(&env, owner, id, new_ids.clone());
    }

    fn redeem(env: Env, id: i128) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        if !read_expired(&env) || !read_paid(&env) {
            panic_with_error!(&env, Error::NotPermitted);
        }

        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let owner = read_owner(&env, id);
        owner.require_auth();

        // send funds to owner address
        let sub_nft = read_sub_nft(&env, id);
        let client = token::Client::new(&env, &read_external_token_provider(&env));
        client.transfer(
            &env.current_contract_address(),
            &owner,
            &i128::from(sub_nft.amount),
        );

        // burn the token
        write_owner(&env, id, None);

        event::redeem(&env, owner, id);
    }

    fn set_external_token_provider(env: Env, contract_addr: Address) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_external_token_provider(&env, contract_addr);
    }

    fn check_paid(env: Env) -> bool {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let paid_cached = read_paid(&env);
        if paid_cached {
            return true;
        }
        let client = token::Client::new(&env, &read_external_token_provider(&env));
        let balance = client.balance(&env.current_contract_address());
        let paid = balance >= i128::from(read_total_amount(&env));
        if paid {
            write_paid(&env, true);
        }
        paid
    }

    fn check_expired(env: Env) -> bool {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let expired_cached = read_expired(&env);
        if expired_cached {
            return true;
        }
        let ledger = env.ledger();
        let expired = ledger.timestamp() >= read_end_time(&env);
        if expired {
            write_expired(&env, true);
            // transfer unclaimed NFTs to the root NFT's owner address
            let last_id = read_supply(&env);
            if last_id > 0 {
                let to = read_owner(&env, 0);
                let contract_addr = &env.current_contract_address();
                for i in 1..last_id {
                    let owner = read_owner(&env, i);
                    if owner == contract_addr.clone() {
                        write_owner(&env, i, Some(to.clone()));
                        event::transfer(&env, contract_addr.clone(), to.clone(), i);
                    }
                }
            }
        }
        expired
    }
}
