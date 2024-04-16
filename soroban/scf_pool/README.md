# TC Loan Smart Contract

This smart contract is for managing loan offers for the tokenized certificates (TC) of the `scf_soroban` smart contract. 

## Functions
* `initialize`: Set the admin, and also specify the address for a token that can be used to exchange for a TC during loaning (e.g. USDC contract address).
* `create_offer`: Offer an `amount` of tokens for a specific TC. The offered amount is transferred to the smart contract and will be transferred back to the offerer if the offer is cancelled. The system calling the smart contract is responsible for generating the `offer_id` and storing it.
* `expire_offer`: Can be called by the admin or the creator of a given offer. Cancels the offer and returns the offered tokens to the offerer.
* `accept_offer`: Must be called by the owner of the TC targeted by the offer. Transfers the TC to the offerer, and transfers the offered tokens from the smart contract to the TC's original owner.
