use soroban_sdk::{contractimpl, Env, Symbol, Address, String, panic_with_error};
use crate::interface::{NonFungibleTokenTrait, WriteType};
use crate::admin::{read_administrator, write_administrator, has_administrator};
use crate::metadata::{write_name, write_symbol, read_name, read_symbol, read_token_uri, write_token_uri};
use crate::balance::{increment_supply, read_supply};
use crate::owner::{read_owner, write_owner, check_owner};
use crate::approval::{write_approval, read_approval, write_approval_all, read_approval_all};
use crate::event;
use crate::errors::Error;
use crate::order_info::{get_order_info,set_order_info};

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
        read_administrator(&env)
    }

    fn set_admin(env: Env, new_admin: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    fn name(env: Env) -> String {
        read_name(&env)
    }

    fn symbol(env: Env) -> Symbol {
        read_symbol(&env)
    }

    fn token_uri(env: Env, id: i128) -> String {
        read_token_uri(&env, id)
    }

    fn appr(env: Env, owner: Address, operator: Address, id: i128) {
        owner.require_auth();
        check_owner(&env, &owner, id);

        write_approval(&env, id, Some(operator.clone()));
        event::approve(&env, operator, id);
    }

    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool) {
        owner.require_auth();
        write_approval_all(&env, owner.clone(), operator.clone(), approved);
        event::approve_all(&env, operator, owner)
    }

    fn get_appr(env: Env, id: i128) -> Address {
        read_approval(&env, id)
    }

    fn is_appr(env: Env, owner: Address, operator: Address) -> bool {
        read_approval_all(&env, owner, operator)
    }

    fn owner(env: Env, id: i128) -> Address {
        read_owner(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, id: i128) {
        check_owner(&env, &from, id);
        from.require_auth();
        write_owner(&env, id, Some(to.clone()));
        event::transfer(&env, from, to, id);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128) {
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
        let admin = read_administrator(&env);
        admin.require_auth();
        
        let id = read_supply(&env) + 1;
        write_owner(&env, id, Some(to.clone()));
        increment_supply(&env);

        write_token_uri(&env, id, uri);
        write_NFT_info(&env, id, uri);

        event::mint(&env, to, id)
    }

    fn mint_original(env: Env, to: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();
        
        let id = read_supply(&env);
        let amount = get_order_info(&env);
        write_owner(&env, id, Some(to.clone()));
        increment_supply(&env);
        write_token_uri(&env, id, uri);
        write_NFT_info(&env, id, id, amount.totalAmount);

        event::mint(&env, to, id)
    }

    fn burn(env: Env, id: i128) {
        let admin = read_administrator(&env);
        admin.require_auth();

        let from = read_owner(&env, id);
        write_owner(&env, id, None);

        event::burn(&env, from, id);
    }
    //TODO
    fn split(env: Env, id: i128){
        let admin = read_administrator(&env);
        admin.require_auth();
        
        let from = read_owner(&env, id);
        write_owner(&env, id, None);

        event::burn(&env, from, id);
    }
}
