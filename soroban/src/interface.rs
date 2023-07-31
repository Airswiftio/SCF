use soroban_sdk::{Address, Env, Symbol, String};

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
    // Metadata interface
    // --------------------------------------------------------------------------------

    // Get the name for this token.
    fn name(env: Env) -> String;

    // Get the symbol for this token.
    fn symbol(env: Env) -> Symbol;

    // Get the uniform resource identifier for token "id".
    fn token_uri(env: Env, id: i128) -> String;

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------

    /// Allows "operator" to manage token "id" if "owner" is the current owner of token "id".
    /// Emit event with topics = ["appr", operator: Address], data = [id: i128]
    fn appr(
        env: Env,
        owner: Address,
        operator: Address,
        id: i128,
    );

    /// If "approved", allows "operator" to manage all tokens of "owner"
    /// Emit event with topics = ["appr_all", operator: Address], data = [owner: Address]
    fn appr_all(
        env: Env,
        owner: Address,
        operator: Address,
        approved: bool,
    );

    /// Returns the identifier approved for token "id".
    fn get_appr(env: Env, id: i128) -> Address;

    /// If "operator" is allowed to manage assets of "owner", return true.
    fn is_appr(
        env: Env,
        owner: Address,
        operator: Address,
    ) -> bool;

    /// Get the balance of "id".
    //fn balance(env: Env, owner: Address) -> i128;

    /// Get the owner of "id" token.
    fn owner(env: Env, id: i128) -> Address;

    /// Transfer token "id" from "from" to "to.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        id: i128,
    );

    /// Transfer token "id" from "from" to "to", consuming the allowance of "spender".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        id: i128,
    );

    /// If authorized as the administrator, mint token "id" with URI "uri".
    /// Emit event with topics = ["mint", to: Address], data = [uri: String]
    fn mint(
        env: Env,
        to: Address,
        uri: String,
    );

    fn mint_original(env: Env, to: Address);
    
    fn split(env: Env, id: i128);

    /// If "admin" is the administrator or the token owner, burn token "id" from "from".
    /// Emit event with topics = ["burn", from: Address], data = [id: i128]
    fn burn(env: Env, id: i128);

    // --------------------------------------------------------------------------------
    // Implementation Interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract with "admin" as administrator, "name" as the name, and
    /// "symbol" as the symbol.
    fn initialize(
        e: Env,
        admin: Address,
        name: String,
        symbol: Symbol,
    );
}

pub enum WriteType {
    Add,
    Remove,
}