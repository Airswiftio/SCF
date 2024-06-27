# Argentina Case Liquidity Pool

## Notes
* This smart contract involves an "external token" and a "tokenized certificate". 
    * The "external token" represents funds used for creating and paying off loans. It could be a liquidity pool token or stablecoin, as long as it fulfills the token interface.
    * The "tokenized certificate" (TC) is a token from the `argentina_pledge` smart contract which represents an invoice on the SCF platform. It is non-fungible, and used as collateral for loans in this liquidity pool.
* A "loan" is expressed relative to the external token. For example, the "creditor" is the one who gives external tokens and receives a TC, while the "borrower" is the one who receives external tokens in exchange for the TC. 

## Steps
1. Initialize the smart contract using `initialize`. The "fee_percent" parameter, if set to a value above 0, will increase the amount needed to pay off the loan. 
2. The admin sets up a whitelist of trusted instances of the `argentina_pledge` TC smart contract using `add_whitelisted_tc` and `remove_whitelisted_tc`. Loan offers can only be created if the TC smart contract address is whitelisted.
3. The creditor calls `create_loan_offer` to offer to loan external tokens to a TC holder in exchange for their TC as collateral. To create a loan offer, the creditor must transfer external tokens to the smart contract equal to the "amount" value associated with that TC.
    * The creditor can retrieve their external tokens from the smart contract by cancelling the loan offer. `cancel_loan_offer` can be called by the same creditor as long as the offer hasn't been accepted yet.
    * The pool's payoff fee percentage can be changed by the admin via `set_rate`. This pool-wide fee determines the payoff fee of a loan when the loan is created. The loan's payoff fee percentage will not change after the loan is created, even if the `set_fee_percent` is used to change the pool-wide fee value.
4. The owner of the TC (borrower) can accept using `accept_loan_offer`. This transfers the external tokens to the borrower and transfers ownership of the TC to the smart contract during the duration of the loan.
5. The borrower is now able to freely use the external tokens during the duration of the loan. 
6. Upon the end of the loan period, there are two options.
    * Normal operation: The borrower must use `payoff_loan` to send external tokens to the creditor, receiving their originally owned TC in return. If the loan fee percentage is greater than 0, the borrower must pay back more external tokens than they originally received from the creditor. 
    * Loan default: If the loan still hasn't been paid back (a grace period may be applicable, depending on the external system using this smart contract), the admin can call `default_loan` to transfer the borrowed TC to the creditor. The creditor can then redeem the TC to recover their funds.
