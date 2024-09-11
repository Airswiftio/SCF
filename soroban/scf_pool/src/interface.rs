use crate::storage_types::Offer;
use soroban_sdk::{Address, BytesN, Env, Vec};

pub trait OfferPoolTrait {
    /// Initialize the contract with an admin
    fn initialize(e: Env, admin: Address);

    /// Get the current admin of the contract
    fn admin(e: Env) -> Address;

    /// Get the current version of the contract
    fn version() -> u32;

    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Set the contact admin. Must be called by the current admin.
    fn set_admin(e: Env, admin: Address);

    /// Add support for an external token, like a liquidity pool token. Must be called by the admin.
    fn add_ext_token(e: Env, ext_token: Address);

    /// Remove support for an external token. Must be called by the admin.
    fn remove_ext_token(e: Env, ext_token: Address);

    /// Upgrade the smart contract with a new WASM hash. Must be called by the admin.
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);

    // --------------------------------------------------------------------------------
    // Pool interface
    // --------------------------------------------------------------------------------

    /// Create an offer against a TC. The caller (from) transfers liquidity tokens to the smart contract equal to the value of the TC.
    /// Emit event with topics = ["create", from: Address, amount: i128, fee: i128], data = [offer_id: i128]
    fn create_offer(
        e: Env,
        from: Address,
        ext_token: Address,
        amount: i128,
        fee: i128,
        tc_contract: Address,
        tc_id: i128,
    ) -> i128;

    /// Cancel a offer by expiring it. Caller must be the user who created the request (the from of the offer).
    /// Transfers the liquidity tokens back to the caller (from).
    /// Emit event with topics = ["expire", from: Address], data = [offer_id: i128]
    fn expire_offer(e: Env, from: Address, offer_id: i128);

    /// get an offer by offer_id, anyone can get the offer information with offer id.
    /// If offer is not found, will return a error for empty offer.
    fn get_offer(e: Env, offer_id: i128) -> Offer;

    /// Accept an offer. The caller (to) must own the TC.
    /// Transfers the TC to the creditor ("from" in the offer), and liquidity tokens equal to the offer amount are sent from the smart contract to the caller ("to").
    /// Emit event with topics = ["accept", to: Address], data = [offer_id: i128]
    fn accept_offer(e: Env, to: Address, offer_id: i128);

    /// Close an offer. The caller must be the user who created the offer (creditor, offer.from).
    /// Transfers the (TC value - offer amount - fee) to the recipient set during accept_offer.
    /// Emit event with topics = ["close", from: Address, offer_id: i128], data = [amount: i128]
    fn close_offer(e: Env, offer_id: i128);

    /// Get all supported external tokens, and their associated pool token addresses.
    fn get_ext_tokens(e: Env) -> Vec<Address>;

    /// Get the recipient of an offer. The recipient is the address that will receive the remaining loan amount upon TC maturity.
    fn recipient(e: Env, offer_id: i128) -> Address;
}
