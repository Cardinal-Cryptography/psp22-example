#!/usr/bin/env bash

set -euo pipefail

function assert_proper_cargo_contract {
  local cargo_version=$(cargo contract --version)
  if [ "$cargo_version" != "cargo-contract 1.5.0-2b17587-x86_64-unknown-linux-gnu" ]; then
    error "Detected incorrect cargo-contract version: ${cargo_version}. Please install rev=2b17587 that contains required bug fixes."
  fi
}

function get_timestamp {
  echo "$(date +'%Y-%m-%d %H:%M:%S')"
}

function error {
  echo -e "[$(get_timestamp)] [ERROR] $*"
  exit 1
}

function log_progress {
  bold=$(tput bold)
  normal=$(tput sgr0)
  echo "[$(get_timestamp)] [INFO] ${bold}${1}${normal}"
}

NODE_URL=""
AUTHORITY_SEED=""

if [ -z "${NODE_URL}" ]; then
  log_progress "NODE_URL var not set. Using default ws://localhost:9944"
  NODE_URL="ws://localhost:9944"
fi

if [ -z "$AUTHORITY_SEED" ]; then
  log_progress "AUTHORITY_SEED var not set. Using default //Alice"
  AUTHORITY_SEED="//Alice"
fi
  

TOKEN_NAME="TEST0"
TOKEN_SYMBOL="T0"
DECIMALS=12
TOTAL_SUPPLY=1000000

CONTRACT_ADDRESS=""

function build_contract {
  cargo +nightly contract build --quiet --release 1> /dev/null 2> /dev/null
}

function deploy_contract {
  CONTRACT_ADDRESS=$(cargo +nightly contract instantiate --url "$NODE_URL" --suri "$AUTHORITY_SEED" --constructor new --args \"${TOKEN_NAME}\" \"${TOKEN_SYMBOL}\" ${DECIMALS} ${TOTAL_SUPPLY} --skip-confirm)
  # If we initialize any other contracts in the constructor, there will 
  # be multiple new contract addresses emitted. At the end of execution, 
  # `cargo contract` prints the first one, rather than the last one, 
  # so we have to extract it from the events.
  CONTRACT_ADDRESS=$(echo "$CONTRACT_ADDRESS" | grep -A3 "Event Contracts ➜ Instantiated" | grep contract | tail -1 | cut -d ' ' -f11)
  if [ -z "$CONTRACT_ADDRESS" ]; then
    error "Contract address empty"
  fi
  log_progress "Token contract address: ${CONTRACT_ADDRESS}"
  jq -n \
    --arg contract_address "${CONTRACT_ADDRESS}" \
    '{ "contract_address" : $contract_address }' > addresses.json
}

assert_proper_cargo_contract

log_progress "Building contract.."
build_contract || error "Failed to build contract"

log_progress "Deploying contract.."
deploy_contract || error "Failed to deploy contract"

exit $?
