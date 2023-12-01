use soroban_sdk::{Address, Env, String};

pub trait LiquidityPoolTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract with "admin" as administrator.
    /// token_wasm_hash is for automatically deploying a liquidity token contract.
    /// ext_token_* parameters are for the wrapped USDC contract.
    /// rate is the default rate for paying off loans, expressed as a percentage.
    fn initialize(
        e: Env,
        admin: Address,
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,
        rate: u32,
    );

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(e: Env, new_admin: Address);

    /// Set the liquidity pool's return rate.
    fn set_rate(e: Env, new_rate: u32);

    // --------------------------------------------------------------------------------
    // Pool interface
    // --------------------------------------------------------------------------------

    /// Deposit USDC into the contract in exchange for a corresponding number of liquidity tokens minted to the "from" address.
    /// Emit event with topics = ["deposit", from: Address], data = [amount: u32]
    fn deposit(e: Env, from: Address, amount: u32);

    /// Withdraw USDC from the contract in exchange for a corresponding number of liquidity tokens burned from the "from" address.
    /// Emit event with topics = ["withdraw", from: Address], data = [amount: u32]
    fn withdraw(e: Env, from: Address, amount: u32);

    /// Create a loan request for a number of liquidity tokens, using a TC as collateral. Caller must be the owner or approved for the TC.
    /// Emit event with topics = ["create_loan_request", from: Address], data = [contract_addr: Address, id: i128]
    fn create_loan_request(e: Env, from: Address, contract_addr: Address, id: i128);

    /// Cancel a loan request. Caller must be the user who created the request.
    /// Emit event with topics = ["cancel_loan_request", from: Address], data = [offer_id: i128]
    fn cancel_loan_request(e: Env, from: Address, offer_id: i128);

    /// Accept a loan request. Upon accepting, the caller transfers liquidity tokens to the borrower equal to the associated TC's value, and the request owner transfers the TC to the caller.
    /// Emit event with topics = ["accept_loan_request", from: Address], data = [offer_id: i128]
    fn accept_loan_request(e: Env, from: Address, offer_id: i128);

    /// Close a loan by returning liquidity tokens to the loaner, and returning the TC to the caller.
    /// If the pool's rate is greater than 0, the amount of liquidity tokens required is higher than the original amount.
    /// Emit event with topics = ["payoff_loan", from: Address], data = [offer_id: i128]
    fn payoff_loan(e: Env, from: Address, offer_id: i128);

    /// Get the rate associated with a loan.
    fn get_rate(e: Env, offer_id: i128) -> u32;

    /// Get the contract address and TC id associated with a loan.
    fn get_tc(e: Env, offer_id: i128) -> (Address, i128);

    /// Get the borrower associated with a loan.
    fn get_borrower(e: Env, offer_id: i128) -> Address;

    /// Get the contract address of the liquidity pool token.
    fn get_liquidity_token(e: Env) -> Address;

    /// Get the contract address and decimals of the USDC contract.
    fn get_ext_token(e: Env) -> (Address, u32);
}
