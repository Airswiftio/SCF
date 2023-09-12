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
   soroban config identity generate --global admin && \
   soroban config identity generate --global acc1 && \
   soroban config identity address admin && \
   soroban config identity address acc1
   ```
2. The addresses for the accounts will be output. Take note of the addresses which will be used to initialize the contract and mint the tokens.
3. If using these accounts on a non-sandbox environment, you will have to fund them with Friendbot first.

## Deploy contracts
These steps assume that you have Futurenet set up already. If not, please refer to https://soroban.stellar.org/docs/getting-started/deploy-to-futurenet#configure-futurenet-in-your-cli
1. `cd` to the contract's subdirectory.
   ```bash
   cd soroban/contract_deployer
   ```
2. Deploy the contract.
   ```bash
   soroban contract deploy \
   --network futurenet \
   --source-account admin \
   --wasm target/wasm32-unknown-unknown/release/contract_deployer.wasm
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
4. The contract code needs to be bumped as well. Use the `soroban contract install` to get the wasm hash.
   ```bash
   soroban contract install \
   --network futurenet \
   --source-account admin \
   --wasm target/wasm32-unknown-unknown/release/contract_deployer.wasm
   ```
5. Bump the contract code, replacing WASM_HASH with the hash output from the previous step.
   ```bash
   soroban contract bump \
   --network futurenet \
   --source-account admin \
   --durability persistent \
   --ledgers-to-expire 6312000 \
   --wasm WASM_HASH
   ```

## Prevent contract expiration 
1. To keep the contract from expiring after some time, you may want to set up a cron job that runs the same command from step 3 on a monthly basis. Replace the CONTRACT_ID and WASM_HASH in the bump.sh scripts located in `contract_deployer` and `scf_pool`.
2. Edit your crontab configuration using `crontab -e`. Add lines such as the following to your crontab, replacing the path to the bump.sh files with your own. These commands run bump.sh on the 1st of every month, which should be sufficiently frequent even on accelerated standalone network ledgers.
   ```
   0 0 1 * * /home/ubuntu/scf/soroban/contract_deployer/bump.sh
   0 0 1 * * /home/ubuntu/scf/soroban/scf_pool/bump.sh
   ```
