use soroban_sdk::{Address, Env, String};

use crate::storage_types::HashMetadata;

pub trait TokenizedCertificateTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract with "admin" as administrator
    fn initialize(e: Env, admin: Address, ext_token_address: Address, ext_token_decimals: u32);

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(e: Env, new_admin: Address);

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------

    /// Admin calls this function. Minted TC belongs to contract.
    /// Emit event with topics = ["mint", to: Address], data = [uri: String]
    fn mint(e: Env, amount: u32, po_hash: String, invoice_hash: String, bol_hash: String);

    /// Transfer token 'id' between specified 'from' and 'to' addresses
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer(e: Env, from: Address, to: Address, id: i128);

    /// Transfer token 'id' between specified 'from' and 'to' addresses, consuming the allowance of "spender".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, id: i128);

    /// Allows "operator" to manage token "id" if "owner" is the current owner of token "id".
    /// Emit event with topics = ["appr", operator: Address], data = [id: i128]
    fn appr(e: Env, owner: Address, operator: Address, id: i128);

    /// If "approved", allows "operator" to manage all tokens of "owner"
    /// Emit event with topics = ["appr_all", operator: Address], data = [owner: Address]
    fn appr_all(e: Env, owner: Address, operator: Address, approved: bool);

    /// Returns the identifier approved for token "id".
    fn get_appr(e: Env, id: i128) -> Address;

    /// If "operator" is allowed to manage assets of "owner", return true.
    fn is_appr(e: Env, owner: Address, operator: Address) -> bool;

    /// Transfers USDC to the contract address, and transfers ownership of the TC to the caller.
    fn pledge(e: Env, from: Address, id: i128);

    /// Burns the TC in exchange for its 'amount' value in USDC to be sent to the owner.
    fn redeem(e: Env, to: Address, id: i128);

    /// Gets the 'amount' value of a TC
    fn get_amount(e: Env, id: i128) -> u32;

    /// Returns the owner of a given TC
    fn get_owner(e: Env, id: i128) -> Address;

    /// Returns the invoice, PO, and BOL hashes of a given TC
    fn get_metadata(e: Env, id: i128) -> HashMetadata;
}
