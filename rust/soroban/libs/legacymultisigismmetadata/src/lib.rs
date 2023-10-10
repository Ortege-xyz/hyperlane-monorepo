#![no_std]
use soroban_sdk::{vec, Address, Bytes, BytesN, Env, Vec};

const MERKLE_ROOT_OFFSET: u32 = 0;
const MERKLE_INDEX_OFFSET: u32 = 32;
const ORIGIN_MAILBOX_OFFSET: u32 = 36;
const MERKLE_PROOF_OFFSET: u32 = 68;
const THRESHOLD_OFFSET: u32 = 1092;
const SIGNATURES_OFFSET: u32 = 1093;
const SIGNATURE_LENGTH: u32 = 65;

/**
 * Format of metadata:
 * [   0:  32] Merkle root
 * [  32:  36] Root index
 * [  36:  68] Origin mailbox address
 * [  68:1092] Merkle proof
 * [1092:1093] Threshold
 * [1093:????] Validator signatures, 65 bytes each, length == Threshold
 * [????:????] Addresses of the entire validator set, left padded to bytes32
 */
pub struct LegacyMultisigIsmMetadata;

impl LegacyMultisigIsmMetadata {
    pub fn root(_metadata: Bytes) -> BytesN<32> {
        return _metadata
            .slice(MERKLE_ROOT_OFFSET..MERKLE_INDEX_OFFSET)
            .try_into()
            .expect("LegacyMultisigIsmMetadata: Error to get root");
    }

    pub fn index(_metadata: Bytes) -> u32 {
        let bytes_index = _metadata.slice(MERKLE_INDEX_OFFSET..ORIGIN_MAILBOX_OFFSET);

        let mut slice: [u8; 4] = [0; 4];
        bytes_index.copy_into_slice(&mut slice);
        let index = u32::from_be_bytes(slice);

        return index;
    }

    pub fn origin_mailbox(_metadata: Bytes) -> BytesN<32> {
        return _metadata
            .slice(ORIGIN_MAILBOX_OFFSET..MERKLE_PROOF_OFFSET)
            .try_into()
            .expect("LegacyMultisigIsmMetadata: Error to get root");
    }

    pub fn proof(env: Env, _metadata: Bytes) -> Vec<BytesN<32>> {
        let proofs_bytes = _metadata.slice(MERKLE_PROOF_OFFSET..THRESHOLD_OFFSET);

        let mut proofs = vec![&env];
        for i in 0..32u32 {
            let proof: BytesN<32> = proofs_bytes
                .slice(i..(i + 1 * 32))
                .try_into()
                .expect("Error to get proof");
            proofs.insert(i, proof);
        }

        return proofs;
    }

    pub fn threshold(_metadata: Bytes) -> u8 {
        let threshold = _metadata
            .get(THRESHOLD_OFFSET)
            .expect("Error to get threshold");
        return threshold;
    }

    pub fn signature_at(_metadata: Bytes, _index: u32) -> Bytes {
        let _start = SIGNATURES_OFFSET + (_index * SIGNATURE_LENGTH);
        let _end = _start + SIGNATURE_LENGTH;

        return _metadata.slice(_start.._end);
    }

    pub fn validator_at(env: Env, _metadata: Bytes, _index: u32) -> Address {
        let _start = Self::_validators_offset(_metadata.clone()) + (_index * 32) + 12;
        let _end = _start + 20;

        let mut address = [0u8; 32];

        let eth_address_bytes: BytesN<20> = _metadata
            .slice(_start.._end)
            .try_into()
            .expect("Error to get validator");

        address[12..32].copy_from_slice(&eth_address_bytes.to_array());

        return Address::from_contract_id(&BytesN::from_array(&env, &address));
    }

    pub fn validators(_metadata: Bytes) -> Bytes {
        return _metadata.slice(Self::_validators_offset(_metadata.clone())..);
    }

    pub fn validator_count(_metadata: Bytes) -> u32 {
        return (_metadata.len() - Self::_validators_offset(_metadata.clone())) / 32;
    }

    fn _validators_offset(_metadata: Bytes) -> u32 {
        return SIGNATURES_OFFSET + (Self::threshold(_metadata.clone()) as u32 * SIGNATURE_LENGTH);
    }
}
