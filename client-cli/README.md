# Command line client for interacting with PSP22 contracts

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
  -m <CONTRACT_METADATA>      Path to contract's metadata [default: ../contracts/target/ink/metadata.json]
  -h, --help                  Print help information
  ```

The client in this repository is built to interact with an instance of PSP22 token contract that implements the following OpenBrush traits:  
* Mint/Burn
* Metadata
* Ownable

The above functionalities add respective methods to the contract which are then exposed in this CLI tool (see `psp22-client psp22 --help` and `psp22-client ownable --help`).

The tool can communicate with any network via setting `--node`/`-n` configuration parameter:

```shell
psp22-client -n ws://localhost:9943 ...
```

It allows for directing the queries at specific contract address via `-c` param:
```shell
psp22-client -n ws://localhost:9943 -c 5FBHFs7eG9ZkajQgkwTA39WZ8yYXq43cAPyro1Pkkb6JejEy ...
```

## Important

Note the `metadata.json` file available in this repository - it's describing methods, arguments and return types of the contract we will be interacting with. It **must** be compatible with the deployed contract (the file is generated at the time of building the contract) otherwise the interaction may fail.
