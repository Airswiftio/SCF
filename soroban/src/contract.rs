use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, String, panic_with_error, Vec};
use crate::interface::{NonFungibleTokenTrait, WriteType};
use crate::admin::{read_administrator, write_administrator, has_administrator};
use crate::metadata::{write_name, write_symbol, read_name, read_symbol, read_token_uri, write_token_uri};
use crate::sub_nft::{write_sub_nft, read_sub_nft, read_sub_nft_disabled, write_sub_nft_disabled};
use crate::balance::{increment_supply, read_supply};
use crate::owner::{read_owner, write_owner, check_owner};
use crate::approval::{write_approval, read_approval, write_approval_all, read_approval_all};
use crate::storage_types::INSTANCE_BUMP_AMOUNT;
use crate::event;
use crate::errors::Error;
use crate::order_info::{write_order_info, read_total_amount};

#[contract]
pub struct NonFungibleToken;

#[contractimpl]
impl NonFungibleTokenTrait for NonFungibleToken {
    fn initialize(e: Env, admin: Address, name: String, symbol: Symbol) {
        if has_administrator(&e) {
            panic!("already initialized")
        }

        write_administrator(&e, &admin);
        write_name(&e, &name);
        write_symbol(&e, &symbol);
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

    fn owner(env: Env, id: i128) -> Address {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_owner(&env, id)
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
        
        if read_approval_all(&env, from.clone(), spender.clone()) || spender == read_approval(&env, id) {
            write_approval(&env, id, None);

            write_owner(&env, id, Some(to.clone()));

            event::transfer(&env, from, to, id);
        } else {
            panic_with_error!(&env, Error::NotAuthorized)
        }
    }
    //TODO
    fn mint(env: Env, to: Address, uri: String) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();
        
        let id = read_supply(&env) + 1;
        write_owner(&env, id, Some(to.clone()));
        increment_supply(&env);

        write_token_uri(&env, id, uri);
        //write_nft_info(&env, id, uri);

        event::mint(&env, to, id)
    }

    fn mint_original(env: Env, to: Address) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();
        
        let id = read_supply(&env);
        let amount = read_total_amount(&env);
        write_owner(&env, id, Some(to.clone()));
        increment_supply(&env);
        write_token_uri(&env, id, String::from_slice(&env, "test"));
        // TODO: is token uri still needed?
        write_sub_nft(&env, id, id, amount);

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
    //TODO
    fn split(env: Env, id: i128, amounts: Vec<u32>) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        if read_sub_nft_disabled(&env, id) {
            // if the NFT is disabled, it has already been split
            panic_with_error!(&env, Error::NotPermitted);
        }
        let owner = read_owner(&env, id);
        owner.require_auth();
        let admin = read_administrator(&env);

        let root = read_sub_nft(&env, id);
        if amounts.iter().sum::<u32>() > root.amount {
            panic_with_error!(&env, Error::AmountTooMuch);
        }

        let mut remaining = root.amount;
        for amount in amounts {
            let new_id = read_supply(&env) + 1;
            write_sub_nft(&env, new_id, id, amount);
            write_owner(&env, new_id, Some(admin.clone()));
            increment_supply(&env);
            remaining -= amount;
        }

        // if root amount > 0, create another sub nft to represent the remaining amount belonging to original owner
        if remaining > 0 {
            let new_id = read_supply(&env) + 1;
            write_sub_nft(&env, new_id, id, remaining);
            write_owner(&env, new_id, Some(owner));
            increment_supply(&env);
        }

        // disable the original NFT
        write_sub_nft_disabled(&env, id, true);
    }

    fn redeem(env: Env, id: i128) {
        env.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        // check that contract address on ledger has enough USDC
        // burn the NFT by setting owner address to null
    }
}
