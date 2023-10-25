#![no_std]
use ecdsa::ECDSA;
use enumerableset::EnumerableSet;
use iinterchain_security_module::Types;
use legacy_checkpoint::LegacyCheckpointLib;
use legacy_multisig_ism_metadata::LegacyMultisigIsmMetadata;
use merkletree::MerkleTree;
use message::Message;
use ownable::Ownable;
use soroban_sdk::{
    contract, contractimpl, symbol_short, testutils::Address as _, vec, Address, Bytes, BytesN,
    Env, Symbol, Vec,
};
use tiny_keccak::{Hasher, Keccak};

const COMMITMENT: Symbol = symbol_short!("COMMITMEN"); // Purposeful because the symbol only accepts 9 characters
const VALIDATOR: Symbol = symbol_short!("VALIDATOR");
const THRESHOLD: Symbol = symbol_short!("THRESHOLD");

#[contract]
pub struct LegacyMultisigIsm;

#[contractimpl]
impl LegacyMultisigIsm {
    pub fn init(env: Env, owner: Address) {
        Ownable::init(env.clone(), owner);
    }

    pub fn enroll_validator(env: Env, caller: Address, _domain: u32, _validator: Address) {
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller);

        Self::_enroll_validator(env.clone(), _domain, _validator);

        Self::_update_commitment(env.clone(), _domain);
    }

    pub fn enroll_validators(
        env: Env,
        caller: Address,
        _domains: Vec<u32>,
        _validators: Vec<Vec<Address>>,
    ) {
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller);

        let domains_length = _domains.len();
        assert!(
            domains_length == _validators.len(),
            "mismatch domains length"
        );

        for i in 0..domains_length {
            let _domain_validators = _validators.get(i).expect("Error to get validators");
            let validators_length = _domain_validators.len();

            for j in 0..validators_length {
                Self::_enroll_validator(
                    env.clone(),
                    _domains.get(i).expect("Error to get domain"),
                    _domain_validators.get(j).expect("Error to get validator"),
                );
            }

            Self::_update_commitment(env.clone(), _domains.get(i).expect("Error to get domain"));
        }
    }

    pub fn unenroll_validator(env: Env, caller: Address, _domain: u32, _validator: Address) {
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller);

        let mut validator_set = Self::_get_validator_set(env.clone(), _domain);

        assert!(
            validator_set.remove(_validator.contract_id()),
            "not enrolled"
        );

        let _validator_count = validator_set.len();

        let threshold = Self::_get_threshold(env.clone(), _domain);

        assert!(
            _validator_count >= threshold as u32,
            "violates quorum threshold"
        );

        Self::_set_validator_set(env.clone(), _domain, &validator_set);

        Self::_update_commitment(env.clone(), _domain);

        env.events().publish(
            (VALIDATOR, symbol_short!("Unenroll")),
            (_domain, _validator, _validator_count),
        )
    }

    pub fn set_threshold(env: Env, caller: Address, _domain: u32, threshold: BytesN<1>) {
        // _threshold is BytesN<1> because functions don't accepts u8
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller);

        let _threshold = threshold.get_unchecked(1);
        let _validator_count = Self::validator_count(env.clone(), _domain);
        assert!(
            _threshold > 0 && (_threshold as u32) <= _validator_count,
            "mismatch range"
        );

        Self::_set_threshold(env.clone(), _domain, _threshold);

        env.events().publish(
            (THRESHOLD, symbol_short!("SET")),
            (_domain, _threshold as u32),
        );

        Self::_update_commitment(env.clone(), _domain);
    }

    pub fn set_thresholds(env: Env, caller: Address, _domains: Vec<u32>, _thresholds: Bytes) {
        // _thresholds is bytes because vec don't accepts u8
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller.clone());

        let length = _domains.len();
        assert!(length == _thresholds.len(), "mismatch length");

        for i in 0..length {
            let _domain = _domains.get(i).expect("Error to get domain");
            let _threshold = _thresholds.get(i).expect("Error to get threshold");

            Self::set_threshold(
                env.clone(),
                caller.clone(),
                _domain,
                BytesN::from_array(&env, &[_threshold]),
            )
        }
    }

    pub fn is_enrolled(env: Env, _domain: u32, _address: Address) -> bool {
        let _validator_set = Self::_get_validator_set(env.clone(), _domain);
        return _validator_set.contains(_address.contract_id());
    }

    pub fn verify(env: Env, _metadata: Bytes, _message: Message) -> bool {
        assert!(
            Self::_verify_merkle_proof(env.clone(), _metadata.clone(), _message),
            "mismatch merkle"
        );
        assert!(
            Self::_verify_validator_signatures(env.clone(), _metadata.clone(), _message),
            "mismatch merkle"
        );
        return true;
    }

    pub fn validators(env: Env, _domain: u32) -> Vec<Address> {
        let validator_set = Self::_get_validator_set(env.clone(), _domain);
        let validator_count = validator_set.len();
        let mut validators: Vec<Address> = Vec::new(&env);

        for i in 0..validator_count {
            let validator = validator_set.at(i).expect("Error to get validator");
            validators.set(i, Address::from_contract_id(&validator));
        }

        return validators;
    }

    pub fn validator_count(env: Env, _domain: u32) -> u32 {
        let validator_set = Self::_get_validator_set(env.clone(), _domain);
        return validator_set.len();
    }

    pub fn module_type() -> Types {
        return Types::LEGACYMULTISIG;
    }

    fn _enroll_validator(env: Env, _domain: u32, _validator: Address) {
        let zero_address = Address::from_contract_id(&BytesN::from_array(&env, &[0; 32]));
        assert!(_validator != zero_address, "zero address");

        let mut validator_set = Self::_get_validator_set(env.clone(), _domain);

        let address = _validator.contract_id();

        assert!(validator_set.add(address), "already enrolled");

        Self::_set_validator_set(env.clone(), _domain, &validator_set.clone());

        let count = Self::validator_count(env.clone(), _domain);

        env.events().publish(
            (VALIDATOR, symbol_short!("Enrolled")),
            (_domain, count, _validator.clone()),
        );
    }

    fn _update_commitment(env: Env, _domain: u32) -> BytesN<32> {
        let validators = Self::validators(env.clone(), _domain);
        let _threshold = Self::_get_threshold(env.clone(), _domain);

        let _commitment = Self::_calculate_commitment(env.clone(), validators, _threshold);

        Self::_set_commitment(env.clone(), _domain, _commitment.clone());

        env.events()
            .publish((COMMITMENT, symbol_short!("UPDATED")), _commitment.clone());

        return _commitment;
    }

    fn _verify_merkle_proof(env: Env, _metadata: Bytes, _message: Message) -> bool {
        let _calculated_root = MerkleTree::branch_root(
            env.clone(),
            _message.id(env.clone()),
            LegacyMultisigIsmMetadata::proof(env.clone(), _metadata.clone()),
            _message.nonce() as u64,
        );

        return _calculated_root == LegacyMultisigIsmMetadata::root(_metadata.clone());
    }

    fn _verify_validator_signatures(env: Env, _metadata: Bytes, _message: Message) -> bool {
        let _threshold = LegacyMultisigIsmMetadata::threshold(_metadata.clone());
        let _digest: BytesN<32> = {
            let _origin = _message.origin();

            let _validators_bytes = LegacyMultisigIsmMetadata::validators(_metadata.clone());
            let mut _validators: Vec<Address> = vec![&env];
            for i in 0.._validators_bytes.len() {
                let start = i * 32;
                let end = (i + 1) * 32;
                let bytes = _validators_bytes.slice(start..end);
                let bytes32: BytesN<32> = bytes
                    .try_into()
                    .expect("Error to get the validator bytes32");
                let validator_address = Address::from_contract_id(&bytes32);
                _validators.set(i, validator_address);
            }

            let _commitment = Self::_calculate_commitment(env.clone(), _validators, _threshold);

            let _storaged_commitment = Self::_get_commitment(env.clone(), _origin);

            assert!(_commitment == _storaged_commitment, "different commitment");

            LegacyCheckpointLib::digest(
                env.clone(),
                _origin,
                LegacyMultisigIsmMetadata::origin_mailbox(_metadata.clone()),
                LegacyMultisigIsmMetadata::root(_metadata.clone()),
                LegacyMultisigIsmMetadata::index(_metadata.clone()),
            )
        };

        let _validator_count = LegacyMultisigIsmMetadata::validator_count(_metadata.clone());
        let mut _validator_index = 0u32;

        for i in 0.._threshold {
            let _signer = ECDSA::recover(
                env.clone(),
                _digest.clone(),
                LegacyMultisigIsmMetadata::signature_at(_metadata.clone(), i as u32),
            );

            while _validator_index < _validator_count
                && _signer
                    != LegacyMultisigIsmMetadata::validator_at(
                        env.clone(),
                        _metadata.clone(),
                        _validator_index,
                    )
            {
                _validator_index += 1;
            }

            assert!(_validator_index < _validator_count, "different threshold");
            _validator_index += 1;
        }

        return true;
    }

    // Storage access functions
    fn _get_commitment(env: Env, _domain: u32) -> BytesN<32> {
        return env
            .storage()
            .persistent()
            .get(&(COMMITMENT, _domain))
            .unwrap_or(BytesN::from_array(&env, &[0u8; 32]));
    }

    fn _set_commitment(env: Env, _domain: u32, _commitment: BytesN<32>) {
        env.storage()
            .persistent()
            .set(&(COMMITMENT, _domain), &_commitment);
    }

    fn _get_threshold(env: Env, _domain: u32) -> u8 {
        return env
            .storage()
            .persistent()
            .get(&(THRESHOLD, _domain))
            .unwrap_or(0) as u8;
    }

    fn _set_threshold(env: Env, _domain: u32, _threshold: u8) {
        env.storage()
            .persistent()
            .set(&(THRESHOLD, _domain), &(_threshold as u32));
    }

    fn _get_validator_set(env: Env, _domain: u32) -> EnumerableSet {
        return env
            .storage()
            .persistent()
            .get(&(VALIDATOR, _domain))
            .unwrap_or(EnumerableSet::new(env.clone()));
    }

    fn _set_validator_set(env: Env, _domain: u32, set: &EnumerableSet) {
        env.storage().persistent().set(&(VALIDATOR, _domain), set);
    }

    // Help function to calculate the commitment
    fn _calculate_commitment(env: Env, validators: Vec<Address>, _threshold: u8) -> BytesN<32> {
        let mut hasher = Keccak::v256();
        hasher.update(&[_threshold]);

        for index in 0..validators.len() {
            let validator = validators.get(index).expect("Error to get validator");
            let bytes_validator = validator.contract_id();
            hasher.update(&bytes_validator.to_array());
        } // similar to abi.encodePacked(_threshold, _validators)

        let mut _commitment: [u8; 32] = [0; 32];
        hasher.finalize(&mut _commitment);

        let _bytes_commitment = BytesN::from_array(&env, &_commitment);

        return _bytes_commitment;
    }

    // Ownable functions
    pub fn owner(env: Env) -> Address {
        return Ownable::owner(env);
    }

    pub fn renounce_ownership(env: Env, caller: Address) {
        Ownable::renounce_ownership(env, caller);
    }

    pub fn transfer_owner_ship(env: Env, caller: Address, new_owner: Address) {
        Ownable::transfer_owner_ship(env, caller, new_owner);
    }
}
