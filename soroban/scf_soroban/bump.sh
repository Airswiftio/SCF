#!/bin/bash
cd /home/ubuntu/SCF_contracts/soroban/scf_soroban/
soroban contract bump --wasm target/wasm32-unknown-unknown/release/scf_soroban.wasm --network futurenet --source-account admin --durability persistent --ledgers-to-expire 6312000