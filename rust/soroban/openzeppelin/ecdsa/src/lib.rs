#![no_std]
use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
use secp256k1::{Message, Secp256k1};
use sha3::{Digest, Keccak256};
use soroban_sdk::{bytes, Address, Bytes, BytesN, Env};

pub struct ECDSA;
impl ECDSA {
    /**
     * Rust version of ecrecover precompiled function
     * Returns the address in sorobank address format
     */
    pub fn ecrecover(env: Env, hash: [u8; 32], r: [u8; 32], s: [u8; 32], v: u8) -> Address {
        // Initialize the secp256k1 context
        let secp = Secp256k1::new();

        // Create a compact signature
        let mut signature_bytes = [0u8; 64];
        signature_bytes[0..32].copy_from_slice(&r);
        signature_bytes[32..64].copy_from_slice(&s);

        //In secp256k1, the recovery id (recid) is typically a value between 0 and 3.
        //However, in Ethereum-style signatures, the recid is often 27 or 28 instead of 0 or 1.
        let normalized_v = v - 27;
        let id = RecoveryId::from_i32(normalized_v as i32).expect("ECDSA: Recovery Id is invalid");

        // Create a signature object
        let signature = RecoverableSignature::from_compact(&signature_bytes, id)
            .expect("Signature creation failed");

        // Create a message from the hash
        let message = Message::from_slice(&hash).unwrap();
        // Recover the public key
        let public_key = secp.recover_ecdsa(&message, &signature);

        let pk = public_key.expect("ECDSA: Invalid signature");

        // Remove the 0x04 byte to get the 64 bytes representation of the public key
        let key = pk.serialize_uncompressed();
        let pub_key_bytes = &key[1..];

        // Hash the public hey in Keccak256 function to get the 32 bytes of address
        let address_hash: [u8; 32] = Keccak256::digest(&pub_key_bytes).into();

        // The Ethereum address is the last 20 bytes of the Keccak256 hash of the public key.
        let eth_address = &address_hash[(address_hash.len() - 20)..];

        // Padding the first 12 bytes with 0 to complete the 32 bytes
        let mut address = [0u8; 32];
        address[0..12].copy_from_slice(&[0u8; 12]);
        address[12..32].copy_from_slice(&eth_address);

        return Address::from_contract_id(&BytesN::from_array(&env, &address));
    }

    /**
     * @dev Returns the address that signed a hashed message (`hash`) with
     * `signature`. This address can then be used for verification purposes.
     *
     * The `ecrecover` EVM opcode allows for malleable (non-unique) signatures:
     * this function rejects them by requiring the `s` value to be in the lower
     * half order, and the `v` value to be either 27 or 28.
     *
     * IMPORTANT: `hash` _must_ be the result of a hash operation for the
     * verification to be secure: it is possible to craft signatures that
     * recover to arbitrary addresses for non-hashed data. A safe way to ensure
     * this is by receiving a hash of the original message (which may otherwise
     * be too long), and then calling {toEthSignedMessageHash} on it.
     */
    pub fn recover(env: Env, hash: BytesN<32>, signature: Bytes) -> Address {
        // Check the signature length
        if signature.len() != 65 {
            panic!("ECDSA: Invalid signature length");
        }

        // Divide the signature in r, s and v variables
        let r: BytesN<32> = signature
            .slice(0..32)
            .try_into()
            .expect("ECDSA: Error to to get 'r' value");
        let s = signature.slice(32..64);
        let v: u8 = signature.get(64).expect("ECDSA: Error to get 'v' value");

        // EIP-2 still allows signature malleability for ecrecover(). Remove this possibility and make the signature
        // unique. Appendix F in the Ethereum Yellow paper (https://ethereum.github.io/yellowpaper/paper.pdf), defines
        // the valid range for s in (281): 0 < s < secp256k1n ÷ 2 + 1, and for v in (282): v ∈ {27, 28}. Most
        // signatures from current libraries generate a unique signature with an s-value in the lower half order.
        //
        // If your library generates malleable signatures, such as s-values in the upper range, calculate a new s-value
        // with 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141 - s1 and flip v from 27 to 28 or
        // vice versa. If your library also generates signatures with 0/1 for v instead 27/28, add 27 to v to accept
        // these malleable signatures as well.

        let max_s = bytes!(
            &env,
            0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0
        );

        assert!(s <= max_s, "ECDSA: Invalid signature 's' value");
        assert!(v == 27 || v == 28, "ECDSA: Invalid signature 'v' value");

        let r_array = r.to_array();
        let s_bytes: BytesN<32> = s.try_into().expect("ECDSA: Error to to get 'r' value");
        let s_array = s_bytes.to_array();
        let signer = Self::ecrecover(env, hash.clone().to_array(), r_array, s_array, v);
        return signer;
    }

    /**
     * @dev Returns an Ethereum Signed Message, created from a `hash`. This
     * replicates the behavior of the
     * [eth_sign](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sign) JSON-RPC method.
     * See {recover}.
     */
    pub fn to_eth_signed_message_hash(env: Env, hash: [u8; 32]) -> BytesN<32> {
        let message = b"\x19Ethereum Signed Message:\n32";

        let mut combined_values = [0u8; 60];
        combined_values[0..28].copy_from_slice(message);
        combined_values[28..60].copy_from_slice(&hash);

        let mut _hash = Keccak256::digest(&combined_values);

        return BytesN::from_array(&env, &_hash.into());
    }
}

#[cfg(test)]
mod tests;
