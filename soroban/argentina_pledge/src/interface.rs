use soroban_sdk::{Address, BytesN, Env, Vec};

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
    /// "amount" is the value of the TC in terms of ext_token.
    /// redeem_time is a Unix timestamp representing the date after which the TC can be redeemed.
    /// file_hashes contains the hashes of each relevant file uploaded when creating the order on the platform's backend.
    /// Emit event with topics = ["mint", to: Address], data = [id: u64]
    fn mint(e: Env, amount: u64, redeem_time: u64, file_hashes: Vec<BytesN<32>>) -> u64;

    /// Transfer token 'id' between specified 'from' and 'to' addresses
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: u64]
    fn transfer(e: Env, from: Address, to: Address, id: u64);

    /// Transfers USDC to the contract address, and transfers ownership of the TC to the caller.
    fn pledge(e: Env, from: Address, id: u64);

    /// Burns the TC in exchange for its 'amount' value in USDC to be sent to the owner.
    fn redeem(e: Env, to: Address, id: u64);

    /// Gets the 'amount' value of a TC
    fn get_amount(e: Env, id: u64) -> u64;

    /// Returns the owner of a given TC
    fn get_owner(e: Env, id: u64) -> Address;

    /// Returns the list of file hashes associated with a given TC
    fn get_file_hashes(e: Env, id: u64) -> Vec<BytesN<32>>;

    /// Returns the address and decimals of the ext_token
    fn get_ext_token(e: Env) -> (Address, u32);

    /// Returns the redeem time
    fn get_redeem_time(e: Env, id: u64) -> u64;
}
