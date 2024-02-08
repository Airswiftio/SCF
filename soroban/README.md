# SCF

## Environment
Follow the steps according to https://soroban.stellar.org/docs/getting-started/setup 
* For convenience, the repository contains scripts for deploying and bumping smart contracts. Specify the network name in the `network_name` file, the contents of which will be passed to any `--network` argument in the deploy and bump scripts.
* To automatically bump the contracts, you can call the scripts at regular intervals via crontab. Once per month is enough on official Stellar mainnet/testnet/futurenet. For networks running on local Stellar quickstart images, it may be better to bump once per week due to the quicker ledger time.

## Build
1. ```bash
   cd soroban
   ```
2. ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

## Run Unit Tests
1. ```bash
   cd soroban
   ```
2. ```bash
   cargo test
   ```

## Setup sandbox accounts on Soroban CLI
1. ```bash
   soroban config identity generate admin && \
   soroban config identity generate acc1 && \
   soroban config identity address admin && \
   soroban config identity address acc1
   ```
2. The addresses for the accounts will be output. Take note of the addresses which will be used to initialize the contract and mint the tokens.

## Run on Soroban CLI
1. ```bash
   soroban contract invoke \
   --wasm target/wasm32-unknown-unknown/release/scf_soroban.wasm \
   --id 1 \
   --source admin \
   -- \
   initialize \
   --invoice_num "a" \
   --po_num "a" \
   --total_amount 1000000 \
   --checksum "a" \
   --supplier_name "L1 Supplier" \
   --buyer_name "Buyer Company" \
   --start_date "2023-08-04" \
   --end_date "2024-08-04" \
   --admin [YOUR_ADMIN_ADDRESS]
   ```
2. ```bash
   soroban contract invoke \
   --wasm target/wasm32-unknown-unknown/release/scf_soroban.wasm \
   --id 1 \
   --source admin \
   -- \
   mint_original \
   --to [YOUR_ACC1_ADDRESS]
   ```