use soroban_sdk::{Address, Env, Vec};

pub trait LiquidityPoolTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract with "admin" as administrator.
    /// ext_token_address specifies a token to exchange for the TCs. The ext token does not necessarily need to be the same as the TC's ext token; for example, it could be a liquidity pool token.
    /// fee_percent is the default fee for paying off loans, expressed as a percentage.
    fn initialize(e: Env, admin: Address, ext_token_address: Address, fee_percent: u32);

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(e: Env, new_admin: Address);

    /// Set the additional amount percentage that must be paid back to close a loan.
    fn set_fee_percent(e: Env, new_fee_percentage: u32);

    /// Whitelist a TC contract address to use for loans. Does nothing if the whitelist is already whitelisted.
    fn add_whitelisted_tc(e: Env, tc_addr: Address);

    /// Remove a TC contract address from the whitelist. Does nothing if the address is not in the whitelist.
    fn remove_whitelisted_tc(e: Env, tc_addr: Address);

    // --------------------------------------------------------------------------------
    // Pool interface
    // --------------------------------------------------------------------------------

    /// Create a loan offer against a TC. The caller (creditor) transfers liquidity tokens to the smart contract equal to the value of the TC.
    /// The loan will use the liquidity pool's fee percentage at the time of the offer being created
    fn create_loan_offer(e: Env, from: Address, tc_addr: Address, tc_id: u64) -> u64;

    /// Cancel a loan offer. Caller must be the user who created the request (creditor).
    /// Transfers the liquidity tokens back to the caller.
    fn cancel_loan_offer(e: Env, offer_id: u64);

    /// Accept a loan offer. The caller (borrower) must own the TC or have approval to transfer it.
    /// Transfers the TC to the creditor, and liquidity tokens equal to the associated TC's value are sent from the smart contract to the caller.
    fn accept_loan_offer(e: Env, from: Address, offer_id: u64);

    /// Pay off a loan. The caller (borrower) transfers liquidity tokens to the smart contract.
    /// If the contract's fee percentage is greater than 0, the amount of liquidity tokens required to pay off is higher than the original amount.
    /// The loan offer must be accepted prior to this step.
    fn payoff_loan(e: Env, offer_id: u64);

    /// Close a loan by returning the TC from the creditor to the borrower, then sending the liquidity tokens from the smart contract back to the creditor.
    /// Payoff must be completed prior to this step.
    fn close_loan(e: Env, offer_id: u64);

    /// Get the fee percentage associated with a loan.
    fn get_loan_fee(e: Env, offer_id: u64) -> u32;

    /// Get the loan smart contract's current fee percentage.
    fn get_pool_fee(e: Env) -> u32;

    /// Get the contract address and TC id associated with a loan.
    fn get_loan_tc(e: Env, offer_id: u64) -> (Address, u64);

    /// Get the borrower associated with a loan.
    fn get_loan_borrower(e: Env, offer_id: u64) -> Address;

    /// Get the creditor associated with a loan.
    fn get_loan_creditor(e: Env, offer_id: u64) -> Address;

    /// Get the contract address and decimals of the USDC contract.
    fn get_ext_token(e: Env) -> (Address, u32);

    /// Get the amount required to successfully pay off the loan.
    fn get_payoff_amount(e: Env, offer_id: u64) -> i128;

    /// Get the base amount of the loan
    fn get_loan_amount(e: Env, offer_id: u64) -> i128;

    /// Get the status of a loan
    fn get_loan_status(e: Env, offer_id: u64) -> u32;

    /// Get the whitelisted TC contract addresses.
    fn get_whitelisted_tcs(e: Env) -> Vec<Address>;
}
