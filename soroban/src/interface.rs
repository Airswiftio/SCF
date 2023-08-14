use soroban_sdk::{Address, Env, String, Symbol, Vec};

use crate::storage_types::SplitRequest;

pub trait NonFungibleTokenTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Returns the current administrator
    fn admin(env: Env) -> Address;

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(env: Env, new_admin: Address);

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------

    /// Allows "operator" to manage token "id" if "owner" is the current owner of token "id".
    /// Emit event with topics = ["appr", operator: Address], data = [id: i128]
    fn appr(env: Env, owner: Address, operator: Address, id: i128);

    /// If "approved", allows "operator" to manage all tokens of "owner"
    /// Emit event with topics = ["appr_all", operator: Address], data = [owner: Address]
    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool);

    /// Returns the identifier approved for token "id".
    fn get_appr(env: Env, id: i128) -> Address;

    /// If "operator" is allowed to manage assets of "owner", return true.
    fn is_appr(env: Env, owner: Address, operator: Address) -> bool;

    /// Get the amount associated with "id".
    fn amount(env: Env, id: i128) -> u32;

    /// Get the parent id of "id" token.
    fn parent(env: Env, id: i128) -> i128;

    /// Get the owner of "id" token.
    fn owner(env: Env, id: i128) -> Address;

    /// Get the "disabled" value of "id" token.
    fn is_disabled(env: Env, id: i128) -> bool;

    /// Transfer token "id" from "from" to "to.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer(env: Env, from: Address, to: Address, id: i128);

    /// Transfer token "id" from "from" to "to", consuming the allowance of "spender".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128);

    /// If authorized as the administrator, mint token "id" with URI "uri".
    /// Emit event with topics = ["mint", to: Address], data = [uri: String]
    //fn mint(env: Env, to: Address, uri: String);

    fn mint_original(env: Env, to: Address);

    /// Split a token into a number of sub-tokens based on the amounts listed. Will fail if the sum of amounts is greater than the original.
    fn split(env: Env, id: i128, splits: Vec<SplitRequest>) -> Vec<i128>;

    /// Burn a specified NFT and transfer funds to the owner.
    fn redeem(env: Env, id: i128);

    /// If "admin" is the administrator or the token owner, burn token "id" from "from".
    /// Emit event with topics = ["burn", from: Address], data = [id: i128]
    fn burn(env: Env, id: i128);

    /// checks usdc balance, and unlocks redemption if the balance >= requirement
    fn check_paid(env: Env) -> bool;

    /// use env timestamp and check against stored expiry time
    fn check_expired(env: Env) -> bool;

    /// set the contract address for the external token (e.g. USDC)
    fn set_external_token_provider(env: Env, contract_addr: Address);

    /// retrieves a pending split request for a given token "id"
    fn pending_sign_off(env: Env, id: i128) -> SplitRequest;

    /// approve and receive the NFT according to SplitRequest for "id"
    fn sign_off(env: Env, id: i128);

    // --------------------------------------------------------------------------------
    // Implementation Interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract with "admin" as administrator, "name" as the name, and
    /// "symbol" as the symbol.
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
    );
}
