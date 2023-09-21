# MerkleTree

This is a version of [hyperlane's MerkleTree lib](https://github.com/hyperlane-xyz/hyperlane-monorepo/blob/main/solidity/contracts/libs/Merkle.sol) write in rust to work in soroban blockchain.

This lib create the tree and the root of it.

## Functions

### Insert

Insert a new value to tree.

### Root

Return the root of tree.

### Branch_root

Calculates and returns the merkle root for the given leaf.

### Keccak256

Helper function to calculate the keccak256 hash of values.

## Run on Sandbox

First you need to build the contract using the command below, then you can run the contract on [sandbox](https://soroban.stellar.org/docs/getting-started/hello-world#run-on-sandbox)

The contract is only a helper for tests, therefore it have basic functions.

```bash
soroban contract build
```

To check the function parameters you can run the command below, change the `FUNCTION_NAME` to the name of the function you want to get the parameters.

```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/merkletree.wasm \
    --id 1 \
    -- \
    FUNCTION_NAME \
    --help
```

### insert

To run the function you can run the example below, it will verify if 1 is part of merkle tree.

```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/merkletree.wasm \
    --id 1 \
    -- \
    insert \
    --node efefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefef
```

### get_root

This function use the data stored in blockchain state, so it will give the same root because the data is 0.

```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/merkletree.wasm \
    --id 1 \
    -- \
    get_root
```

## Test

To run the tests, run the command below:

```
cargo test
```
