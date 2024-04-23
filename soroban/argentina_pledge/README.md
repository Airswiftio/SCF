# Argentina Case Tokenized Certificate

## Steps
1. Initialize the contract using `initialize`. During this step, an external token (such as USDC) must be specified.
2. The admin account calls `mint` to create a tokenized certificate (TC) with some relevant information. Upon minting, this TC belongs to the smart contract so it cannot be used until the `pledge` function is called on it.
3. A user calls `pledge`, trading a given TC's "amount" value in USDC for ownership of that TC.
4. The `redeem` function can be called by the TC's owner upon passing the TC's redeem_time. It burns the TC and sends its "amount" value from the smart contract to the caller.

## Notes
This smart contract contains other methods which gives it functionality like that of an NFT. The `transfer` function allows it to be traded and loaned for USDC in the argentina_pool smart contract.