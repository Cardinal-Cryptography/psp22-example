# Testing

## Prerequisities

1. Rust nightly.
2. `cargo contract` with bug fixes for escaping strings in the arguments `cargo install --git https://github.com/paritytech/cargo-contract.git --rev 2b1758756de59bd81e7bed5f8429d364f281cb9a --force`

## Instructions

1. Run local aleph network (easiest is to check out the `aleph-node` repository and run `./scripts/run_nodes.sh` in the root folder).
2. Execute `./setup.sh` script.
3. If successful - contract address is printed to the console and stored in `addresses.json` file.