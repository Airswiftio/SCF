
use soroban_sdk::{Address,Env, BytesN};
use crate::storage_types::{Offer};

pub trait OfferPoolTrait {
    fn initialize(
        e: Env,
        admin: Address,
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,);
        

    // --------------------------------------------------------------------------------
    // Pool interface
    // --------------------------------------------------------------------------------

    // /// Deposit USDC into the contract in exchange for a corresponding number of liquidity tokens minted to the "from" address with 1:1 ratio.
    // /// Emit event with topics = ["deposit", from: Address], data = [amount: u32]
    fn deposit(e: Env, from: Address, amount: i128);
    // /// Withdraw USDC from the contract in exchange for a corresponding number of liquidity tokens burned from the "from" address with 1:1 ratio.
    // /// Emit event with topics = ["withdraw", from: Address], data = [amount: u32]
    fn withdraw(e: Env, from: Address, amount: i128);

    // /// Create an offer against a TC. The caller (from) transfers liquidity tokens to the smart contract equal to the value of the TC.
    // /// Emit event with topics = ["create_offer", from: Address, amount: i128], data = [offer_id: i128]
    fn create_offer( e: Env, from: Address, offer_id: i128, amount: i128, tc_contract: Address, tc_id: i128,);
    
    // /// Cancel a offer by expiring it. Caller must be the user who created the request (the from of the offer).
    // /// Transfers the liquidity tokens back to the caller (from ).
    // /// Emit event with topics = ["expire_offer", from: Address ], data = [offer_id: i128]
    fn expire_offer(e: Env, from: Address, offer_id: i128);

    // /// get an offer by offer_id, anyone can get the offer information with offer id.
    // /// If offer is not found, will return a error for empty offer.
    fn get_offer(e: Env, offer_id: i128) -> Offer;

    // /// Accept an offer. The caller (to) must own the TC or have approval to transfer it.
    // /// Transfers the TC to the creditor (from in the offer), and liquidity tokens equal to the associated TC's value are sent from the smart contract to the caller (to).
    // /// Emit event with topics = ["accept_offer", to: Address, amount:i128 ], data = [offer_id: i128]
    fn accept_offer(e: Env, to: Address, offer_id: i128);

    // /// Get the contract address of the liquidity pool token.
    fn get_liquidity_token(e: Env) -> Address;

    // /// Get the contract address and decimals of the USDC contract.
    fn get_ext_token(e: Env) -> (Address, u32);

}