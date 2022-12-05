# NOTE: `main` branch of this repository contains contracts compatible with ink! 3.

# Example PSP22 (ERC20) token contract

This repo contains an example implementation of the very basic PSP22 token standard using [OpenBrush library](https://openbrush.io/).

We're purposefully omitting more advanced constructs and focus on the basics to present self-contained, working example of a contract that implements various OpenBrush extensions for PSP22 token that people may find interesting:
* [Token metadata](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/metadata)
* [Ownable modifiers](https://docs.openbrush.io/smart-contracts/ownable)
* [Mint](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/mintable)/[Burn](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/burnable) functionality

The following extensions from the OpenBrush docs are missing:
* [Wrapper](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/wrapper)
* [Flashmint](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/flashmint)
* [Pausable](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/pausable)
* [Capped](https://docs.openbrush.io/smart-contracts/PSP22/Extensions/capped)
* [Timelock](https://docs.openbrush.io/smart-contracts/PSP22/Utils/token-timelock)

but should be possible to add if followed the guidelines presented here.

## Repository structure

This repository contains two directories:
* `/contracts` - containing implementations described above
* `/client-cli` - a command-line tool for interacting with the deployed contract(s). We've tried to set some coding standards that focus on readability of the code which should maximize the educational value of the examples.

## Setup

Before starting the work on the contracts make sure you have the necessary tools installed. Please follow our [official guide](https://docs.alephzero.org/aleph-zero/build/installing-required-tools).

### Building contracts

in `/contracts` subfolder, run `cargo +nightly contract build`.

### Building CLI client.

in `/client-cli` subfolder, run `cargo build --release`. This will create a binary in the standard `/targets/release/psp22-client` path.

### Deploying contracts

You can use any of the online wallets connected to a network that is compatible with the correct version of ink! used. Or you can choose to run a local network.

In `/contracts` subfolder, run `./setup.sh` which, by default, will deploy to your local instance of the network.

## Usage

If you wish to interact with already-deployed contracts you can use the `client-cli`:

```shell
Utilities for interacting with a sample PSP22 contract

Usage: psp22-client [OPTIONS] -c <CONTRACT_ADDRESS> <COMMAND>

Commands:
  psp22    
  ownable  
  help     Print this message or the help of the given subcommand(s)

Options:
  -n, --node <NODE_ADDRESS>   WS endpoint address of the node [default: ws://localhost:9944]
  -c <CONTRACT_ADDRESS>       On-chain address of a contract
  -m <CONTRACT_METADATA>      Path to contract's metadata [default: metadata.json]
  -h, --help                  Print help information
  ```

### Examples

(I have previously deployed the example which now lives under address: `5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy`)

#### **Total supply:**

  ```shell
  psp22_example/client-cli$ ./target/release/psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy psp22 total-supply
Total supply of the underlying token: 1000000
```

#### **Token metadata:**
```shell
psp22_example/client-cli$ ./target/release/psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy psp22 get-token-metadata
TokenMetadata { name: Some("TEST0"), symbol: Some("T0"), decimals: 12 }
```

#### **Balance of:**
```shell
psp22_example/client-cli$ ./target/release/psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy psp22 balance-of -a 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
Balance of d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...): 1000000
```

#### **Transfer:**
```shell
psp22_example/client-cli$ ./target/release/psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy psp22 transfer --recipient 5Eo5ZxVUGbT6D8cfAvAxQFhzt3ZqBEb5oE8KCWR9vjvTPSMy --amount 100000 --account-seed //Alice

psp22_example/client-cli$ ./target/release/psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy psp22 balance-of -a 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
Balance of d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...): 900000

psp22_example/client-cli$ ./target/release/psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy psp22 balance-of -a 5Eo5ZxVUGbT6D8cfAvAxQFhzt3ZqBEb5oE8KCWR9vjvTPSMy
Balance of 78c9dcd1f5af0a3f67dda5d07847ad8aa4e5bd015f88669aa7e4b874040cca5b (5Eo5ZxVU...): 100000
```