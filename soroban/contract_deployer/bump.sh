#!/bin/bash
soroban contract bump --id CONTRACT_ID --network futurenet --source-account admin --durability persistent --ledgers-to-expire 6312000
soroban contract bump --wasm target/wasm32-unknown-unknown/release/contract_deployer.wasm --network futurenet --source-account admin --durability persistent --ledgers-to-expire 6312000