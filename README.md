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
   cd soroban/scf_soroban
   ```
2. ```bash
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
3. ```bash
   soroban contract invoke \
   --wasm target/wasm32-unknown-unknown/release/scf_soroban.wasm \
   --id 1 \
   --source admin \
   -- \
   mint_original \
   --to [YOUR_ACC1_ADDRESS]
   ```
