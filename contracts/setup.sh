#!/usr/bin/env bash

set -euo pipefail

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

NODE_URL=${NODE_URL:-"ws://localhost:9944"}
AUTHORITY_SEED=${AUTHORITY_SEED:-"//Alice"}

log_progress "NODE_URL=${NODE_URL}"
log_progress "AUTHORITY_SEED=${AUTHORITY_SEED}"
  

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

function copy_metadata {
  cp target/ink/metadata.json ../client-cli
}

log_progress "Building contract.."
build_contract || error "Failed to build contract"

log_progress "Deploying contract.."
deploy_contract || error "Failed to deploy contract"

log_progress "Copying contract metadata to client directory.."
copy_metadata || error "Failed to copy contract metadata"

exit $?
