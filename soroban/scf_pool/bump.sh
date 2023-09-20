#!/bin/bash
soroban contract bump --id CONTRACT_ID --network futurenet --source-account admin --durability persistent --ledgers-to-expire 3000000
soroban contract bump --wasm target/wasm32-unknown-unknown/release/pool.wasm --network futurenet --source-account admin --durability persistent --ledgers-to-expire 3000000