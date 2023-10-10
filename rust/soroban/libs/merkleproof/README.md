# MerkleProof Contract

This contract is a version of [openzeppelin's MerkleProof](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol) write in rust to work in soroban blockchain, technical details you can [find here](https://docs.openzeppelin.com/contracts/4.x/api/utils#MerkleProof).

This contract checks whether a leaf is part of the merkle tree.

## Functions

### verify

Returns true if a leaf can be proved to be a part of a Merkle tree defined by root.

### multi_proof_verify

Returns true if the leaves can be simultaneously proven to be a part of a merkle tree defined by root, according to proof and proofFlags.

## Run on Sandbox

First you need to build the contract using the command below, then you can run the contract on [sandbox](https://soroban.stellar.org/docs/getting-started/hello-world#run-on-sandbox)

```bash
soroban contract build
```

To check the function parameters you can run the command below, change the `FUNCTION_NAME` to the name of the function you want to get the parameters.

```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/merkleproof.wasm \
    --id 1 \
    -- \
    FUNCTION_NAME \
    --help
```

### verify

To run the function you can run the example below, it will verify if 1 is part of merkle tree.

```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/merkleproof.wasm \
    --id 1 \
    -- \
    verify \
    --leaf b5d9d894133a730aa651ef62d26b0ffa846233c74177a591a4a896adfda97d22 \
    --proof '[ "c167b0e3c82238f4f2d1a50a8b3a44f96311d77b148c30dc0ef863e1a060dcb6", "673620737675e2755ce8269a99904022d15da8d5843f5aec205cd243ff80240a" ]' \
    --root 4c8ce3926f0293b1d281eac81d8f773dc2a9333e951e7b199adeebc7d7a2ed66
```

### multi_proof_verify

To run the function you can run the example below, it will verify if 3 and 4 is part of merkle tree.

```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/merkleproof.wasm \
    --id 1 \
    -- \
    multi_proof_verify \
    --leaves '[ "2584db4a68aa8b172f70bc04e2e74541617c003374de6eb4b295e823e5beab01",  "c167b0e3c82238f4f2d1a50a8b3a44f96311d77b148c30dc0ef863e1a060dcb6" ]' \
    --proof '[ "1ab0c6948a275349ae45a06aad66a8bd65ac18074615d53676c09b67809099e0", "b5d9d894133a730aa651ef62d26b0ffa846233c74177a591a4a896adfda97d22" ]' \
    --root 4c8ce3926f0293b1d281eac81d8f773dc2a9333e951e7b199adeebc7d7a2ed66 \
    --proof_flags '[ false, false, true ]'
```

## Test

To run the tests, run the command below:

```
cargo test
```
