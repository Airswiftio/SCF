# SCF

## Environment
Follow the steps according to https://soroban.stellar.org/docs/getting-started/setup 

## Build
1. ```bash
   cd soroban
   ```
2. ```bash
   make
   ```

## Run Unit Tests
1. `cd` to each desired sub-directory of the `soroban` folder, and run `cargo test`

## Setup identities on Soroban CLI
1. ```bash
   soroban config identity generate admin && \
   soroban config identity generate acc1 && \
   soroban config identity address admin && \
   soroban config identity address acc1
   ```
2. The addresses for the accounts will be output. Take note of the addresses which will be used to initialize the contract and mint the tokens.
3. If using these accounts on a non-sandbox environment, you will have to fund them with Friendbot first.

## Deploy contracts
These steps assume that you have Futurenet set up already. If not, please refer to https://soroban.stellar.org/docs/getting-started/deploy-to-futurenet#configure-futurenet-in-your-cli
1. `cd` to the contract's subdirectory.
   ```bash
   cd soroban/scf_soroban
   ```
2. Deploy the contract.
   ```bash
   soroban contract deploy \
   --network futurenet \
   --source-account admin \
   --wasm target/wasm32-unknown-unknown/release/scf_soroban.wasm
   ```
3. Bump the contract lifetime. Replace CONTRACT_ID with the output contract ID from the previous step.
   ```bash
   soroban contract bump \
   --network futurenet \
   --source-account admin \
   --durability persistent \
   --ledgers-to-expire 6312000 \
   --id CONTRACT_ID 
   ```
4. To keep the contract from expiring, you may set up a cron job that runs the same command from step 3 on a monthly basis.