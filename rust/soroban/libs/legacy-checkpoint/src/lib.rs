#![no_std]
use ecdsa::ECDSA;
use sha3::{Digest, Keccak256};
use soroban_sdk::{BytesN, Env};
pub struct LegacyCheckpointLib;

impl LegacyCheckpointLib {
    /**
     * @notice Returns the digest validators are expected to sign when signing legacy checkpoints.
     * @param _origin The origin domain of the checkpoint.
     * @param _originMailbox The address of the origin mailbox as bytes32.
     * @return The digest of the legacy checkpoint.
     */
    pub fn digest(
        env: Env,
        _origin: u32,
        _origin_mailbox: BytesN<32>,
        _checkpoint_root: BytesN<32>,
        _checkpoint_index: u32,
    ) -> BytesN<32> {
        let bytes_origin = _origin.to_be_bytes().to_vec();
        let bytes_origin_mailbox = _origin_mailbox.to_array().to_vec();
        let bytes_checkpoint_root = _checkpoint_root.to_array().to_vec();
        let bytes_checkpoint_index = _checkpoint_index.to_be_bytes().to_vec();

        let bytes = [
            &bytes_origin[..],
            &bytes_origin_mailbox[..],
            &bytes_checkpoint_root[..],
            &bytes_checkpoint_index[..],
        ]
        .concat();

        let hash: [u8; 32] = Keccak256::digest(&bytes).into();

        return ECDSA::to_eth_signed_message_hash(env.clone(), hash);
    }

    /**
     * @notice Returns the domain hash that validators are expected to use
     * when signing checkpoints.
     * @param _origin The origin domain of the checkpoint.
     * @param _originMailbox The address of the origin mailbox as bytes32.
     * @return The domain hash.
     */
    pub fn domain_hash(env: Env, _origin: u32, _origin_mailbox: BytesN<32>) -> BytesN<32> {
        let bytes_origin = _origin.to_be_bytes().to_vec();
        let bytes_origin_mailbox = _origin_mailbox.to_array().to_vec();
        let bytes_string = b"HYPERLANE".to_vec();

        let bytes = [
            &bytes_origin[..],
            &bytes_origin_mailbox[..],
            &bytes_string[..],
        ]
        .concat();

        let hash: [u8; 32] = Keccak256::digest(&bytes).into();

        return BytesN::from_array(&env, &hash);
    }
}
