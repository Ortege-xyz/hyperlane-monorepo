#![no_std]
use soroban_sdk::{
    assert_with_error, contract, contracterror, contractimpl, vec, BytesN, Env, Vec,
};
use tiny_keccak::{Hasher, Keccak};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MerkleProofInvalidMultiproof = 1,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn verify(env: Env, proof: Vec<BytesN<32>>, root: BytesN<32>, leaf: BytesN<32>) -> bool {
        return Self::process_proof(env, proof, leaf) == root;
    }

    pub fn multi_proof_verify(
        env: Env,
        proof: Vec<BytesN<32>>,
        proof_flags: Vec<bool>,
        root: BytesN<32>,
        leaves: Vec<BytesN<32>>,
    ) -> bool {
        return Self::process_multi_proof(env, proof, proof_flags, leaves) == root;
    }

    pub fn process_proof(env: Env, proof: Vec<BytesN<32>>, leaf: BytesN<32>) -> BytesN<32> {
        let mut computed_hash = leaf;
        for p in proof {
            computed_hash = Self::_hash_pair(env.clone(), computed_hash, p);
        }
        return computed_hash;
    }

    pub fn process_multi_proof(
        env: Env,
        proof: Vec<BytesN<32>>,
        proof_flags: Vec<bool>,
        leaves: Vec<BytesN<32>>,
    ) -> BytesN<32> {
        let leaves_len = leaves.len();
        let proof_len = proof.len();
        let total_hashes = proof_flags.len();

        // Check proof validity.
        assert_with_error!(
            &env,
            leaves_len + proof_len - 1 == total_hashes,
            Error::MerkleProofInvalidMultiproof
        );

        let mut hashes: Vec<BytesN<32>> = vec![&env];
        let mut leaf_pos: u32 = 0;
        let mut hash_pos: u32 = 0;
        let mut proof_pos: u32 = 0;

        for i in 0..total_hashes {
            let a = {
                let a: BytesN<32>;
                if leaf_pos < leaves_len {
                    a = leaves.get(leaf_pos).expect("Error to get leave");
                    leaf_pos += 1;
                } else {
                    a = hashes.get(hash_pos).expect("Error to get hash");
                    hash_pos += 1;
                }
                a
            };

            let b = {
                let b: BytesN<32>;
                if proof_flags.get_unchecked(i) {
                    if leaf_pos < leaves_len {
                        b = leaves.get(leaf_pos).expect("Error to get leave");
                        leaf_pos += 1;
                    } else {
                        b = hashes.get(hash_pos).expect("Error to get hash");
                        hash_pos += 1;
                    }
                } else {
                    b = proof.get(proof_pos).expect("Error to get proof");
                    proof_pos += 1;
                }
                b
            };

            hashes.insert(i, Self::_hash_pair(env.clone(), a, b));
        }

        if total_hashes > 0{
            assert_with_error!(&env, proof_pos == proof_len, Error::MerkleProofInvalidMultiproof);
            return hashes.get_unchecked(total_hashes -1);
        } else if leaves_len > 0{
            return leaves.get_unchecked(0);
        } else {
            return proof.get_unchecked(0);
        }
    }

    fn _hash_pair(env: Env, a: BytesN<32>, b: BytesN<32>) -> BytesN<32> {
        if a < b {
            return Self::_hash(env, a, b);
        }
        return Self::_hash(env, b, a);
    }

    /// Create a keccak256 hash from 2 values
    fn _hash(env: Env, a: BytesN<32>, b: BytesN<32>) -> BytesN<32> {
        let mut hasher = Keccak::v256();
        let array_a = a.to_array();
        let array_b = b.to_array();
        hasher.update(&array_a);
        hasher.update(&array_b);
        let mut output: [u8; 32] = [0; 32];
        hasher.finalize(&mut output);
        return BytesN::from_array(&env, &output);
    }
}
#[cfg(test)]
mod tests;
