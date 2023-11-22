use soroban_sdk::{Address, Env, String, Vec};

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

    /// Get the data associated with "id".
    fn data(env: Env, id: i128) -> String;

    /// Get all NFTs ids owned by address
    fn get_all_owned(env: Env, address: Address) -> Vec<i128>;

    /// Get the "disabled" value of "id" token.
    fn is_disabled(env: Env, id: i128) -> bool;

    /// Transfer token "id" from "from" to "to.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer(env: Env, from: Address, to: Address, id: i128);

    /// Transfer token "id" from "from" to "to", consuming the allowance of "spender".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128);

    /// Mint the root-level NFT. Will fail if the root-level NFT already exists.
    /// The minted NFT has a value corresponding to the "total_amount" specified in the initialize() function.
    fn mint_original(env: Env, to: Address, data: String);

    /// Split a token into a number of sub-tokens based on the amounts listed. Will fail if the sum of amounts is greater than the original.
    fn split(env: Env, id: i128, splits: Vec<SplitRequest>) -> Vec<i128>;

    /// Burn a specified NFT and transfer funds to the owner.
    fn redeem(env: Env, id: i128);

    /// If "admin" is the administrator or the token owner, burn token "id" from "from".
    /// Emit event with topics = ["burn", from: Address], data = [id: i128]
    fn burn(env: Env, id: i128);

    /// checks whether the payoff step was completed
    fn check_paid(env: Env) -> bool;

    /// use env timestamp and check against stored expiry time
    fn check_expired(env: Env) -> bool;

    /// set the contract address for the external token (e.g. USDC)
    fn set_external_token_provider(env: Env, contract_addr: Address, decimals: u32);

    /// retrieves a pending split request for a given token "id"
    fn recipient(env: Env, id: i128) -> Address;

    /// approve and receive the NFT according to SplitRequest for "id"
    fn sign_off(env: Env, id: i128);

    /// pay off OrderInfo.amount using token
    fn pay_off(env: Env, from: Address);

    // --------------------------------------------------------------------------------
    // Implementation Interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract.
    /// "admin" is the contract administrator.
    /// "invoice_num" and "po_num" are additional identifiers to be used from an external system. The smart contract does not use the values.
    /// "buyer_address" specifies the account that will perform the pay-off step later.
    /// "total_amount" corresponds to the USD value of the invoice.
    /// "start_time" is a Unix timestamp. (resolution: seconds)
    /// "end_time" is also a Unix timestamp. It specifies the maturity date of the invoice, after which the NFTs can be redeemed for USDC or other tokens.
    fn initialize(
        e: Env,
        admin: Address,
        invoice_num: i128,
        po_num: i128,
        buyer_address: Address,
        total_amount: u32,
        start_time: u64,
        end_time: u64,
    );
}
