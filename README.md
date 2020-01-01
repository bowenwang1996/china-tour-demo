# Private Shard Demo

There are two contracts: score-contracts, which emulates a private shard
smart contract that manages some scores, and scholarship contract, which
interacts with the score contract to determine whether a scholarship
should be granted.

## Setup
Let's start the local Near testnet to run the contract on it.

* Clone the [nearprotocol/nearcore](https://github.com/nearprotocol/nearcore);
* Make sure you are in `master` branch, then run
    ```bash
    rm -rf tmp; ./scripts/start_localnet.py --nodocker --debug --home=tmp
    ```
* Make sure you have the newest version of near-shell installed by running:
    ```bash
    npm install -g near-shell
    ```
* Use local development environment for near-shell by setting `NODE_ENV` to `local`.
* Compile two contracts by running `build.sh` in respective directories.

## Deploy contracts

The way to deploy the two contracts are similar. For example, to deploy `score-contract`, run
```bash
near create_account score-contract --masterAccount=<account id for localnet> --homeDir=../nearcore/tmp
near deploy --accountId=score-contract --wasmFile=score-contract/res/score_contract.wasm
```

## Interacting with contracts

To record scores in the private shard, run
```bash
near call score-contract record_score '{"name": "illia","score": 100}' --accountId=score-contract --gas=100000000000000
```

Now to check whether the score qualifies for scholarship, run
```bash
near call scholarship-contract scholarship '{"name":"illia", "block_index":<block_index>}' --accountId=scholarship-contract --gas=10000000000000000
```

where `block_index` should be larger than the block index at which the score recording transaction finishes.

Observe that the `Scholarship granted` is returned.
