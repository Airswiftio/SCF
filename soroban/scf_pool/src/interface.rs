
use soroban_sdk::{Address,Env, BytesN};

pub trait OfferPoolTrait {
    fn initialize(
        e: Env, 
        admin: Address, 
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,);
        
    // fn set_admin(e: Env, new_admin: Address);

    // --------------------------------------------------------------------------------
    // Pool interface
    // --------------------------------------------------------------------------------

    // /// Deposit USDC into the contract in exchange for a corresponding number of liquidity tokens minted to the "from" address.
    // /// Emit event with topics = ["deposit", from: Address], data = [amount: u32]
    // fn deposit(e: Env, from: Address, amount: i128);

    // /// Withdraw USDC from the contract in exchange for a corresponding number of liquidity tokens burned from the "from" address.
    // /// Emit event with topics = ["withdraw", from: Address], data = [amount: u32]
    // fn withdraw(e: Env, from: Address, amount: i128);

    // /// Create a loan offer against a TC. The caller (creditor) transfers liquidity tokens to the smart contract equal to the value of the TC.
    // /// The loan will use the liquidity pool's interest rate at the time of the offer being created
    // fn create_offer(e: Env, from: Address, offer_id: i128, amount: i128, tc_contract: Address, tc_id: i128,);
    
    // /// Cancel a loan offer. Caller must be the user who created the request (creditor).
    // /// Transfers the liquidity tokens back to the caller.
    // fn cancels_offer(e: Env, from: Address, offer_id: i128);
    
    // /// Accept a loan offer. The caller (borrower) must own the TC or have approval to transfer it.
    // /// Transfers the TC to the creditor, and liquidity tokens equal to the associated TC's value are sent from the smart contract to the caller.
    // fn accept_offer(e: Env, token: Address, offer_id: i128);

    // /// Get the contract address of the liquidity pool token.
    // fn get_liquidity_token(e: Env) -> Address;

    // /// Get the contract address and decimals of the USDC contract.
    // fn get_ext_token(e: Env) -> (Address, u32);

}