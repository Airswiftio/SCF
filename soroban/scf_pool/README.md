# TC Offer Pool Smart Contract

This smart contract is for making offers on the tokenized certificates (TC) of the `scf_soroban` smart contract using external tokens such as liquidity pool tokens.

## Steps
1. `initialize`: Set the admin.
2. `add_pool_token`: The admin adds support for an external token, such as a liquidity pool token. 
3. `create_offer`: Offer an `amount` of external tokens for a specific TC. These tokens will be held by the smart contract until the offer is accepted or cancelled.
4. `accept_offer`: Must be called by the owner of the TC targeted by the offer. Transfers the TC to the offerer, and transfers the offered tokens from the smart contract to the TC's original owner.

#### Other functions
* `expire_offer`: Can be called by the admin or the creator of a given offer. Cancels the offer and returns the offered tokens to the offerer.
* `get_offer`: Lookup the details of an offer based on the offer ID.
* `get_ext_tokens`: Return a list of supported external token addresses.