use crate::ECDSA;
use sha3::{Digest, Keccak256};
use soroban_sdk::{bytes, bytesn, Address, BytesN, Env};

#[test]
fn recover_valid_signature_v27() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());

    let test_message = BytesN::from_array(&env, &message_hash.into());

    let signer_bytes = bytesn!(
        &env,
        0x0000000000000000000000002cc1166f6212628A0deEf2B33BEFB2187D35b86c
    );
    let signer = Address::from_contract_id(&signer_bytes);

    let signature = bytes!(&env, 0x5d99b6f7f6d1f73d1a26497f2b1c89b24c0993913f86e9a2d02cd69887d9c94f3c880358579d811b21dd1b7fd9bb01c1d81d10e69f0384e675c32b39643be8921b);

    let recovered_signer = ECDSA::recover(env, test_message, signature);

    assert!(signer == recovered_signer);
}

#[test]
fn recover_valid_signature_v28() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());

    let test_message = BytesN::from_array(&env, &message_hash.into());

    let signer_bytes = bytesn!(
        &env,
        0x0000000000000000000000001E318623aB09Fe6de3C9b8672098464Aeda9100E
    );
    let signer = Address::from_contract_id(&signer_bytes);

    let signature = bytes!(&env, 0x331fe75a821c982f9127538858900d87d3ec1f9f737338ad67cad133fa48feff48e6fa0c18abc62e42820f05943e47af3e9fbe306ce74d64094bdf1691ee53e01c);

    let recovered_signer = ECDSA::recover(env, test_message, signature);

    assert!(signer == recovered_signer);
}

#[test]
#[should_panic]
fn recover_invalid_signature() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());

    let test_message = BytesN::from_array(&env, &message_hash.into());

    let signature = bytes!(&env, 0x431fe75a821c982f9127538858900d87d3ec1f9f737338ad67cad133fa48feff48e6fa0c18abc62e42820f05943e47af3e9fbe306ce74d64094bdf1691ee53d01c);

    let _recovered_signer = ECDSA::recover(env, test_message, signature);
}

#[test]
#[should_panic]
fn recover_invalid_signature_length() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());

    let test_message = BytesN::from_array(&env, &message_hash.into());

    let signature = bytes!(&env, 0x431fe75a821c982f9127538858900d87d3ec1f9f737338ad67cad133fa48feff48e6fa0c18abc62e42820f05943e47af3e9fbe306ce74d64094bdf1691ee53d01cccccccccccc);

    let _recovered_signer = ECDSA::recover(env, test_message, signature);
}

#[test]
#[should_panic]
fn recover_invalid_s_value() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());

    let test_message = BytesN::from_array(&env, &message_hash.into());

    let signature = bytes!(&env, 0x5d99b6f7f6d1f73d1a26497f2b1c89b24c0993913f86e9a2d02cd69887d9c94f7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A11b);

    let _recovered_signer = ECDSA::recover(env, test_message, signature);
}

#[test]
#[should_panic]
fn recover_invalid_v_value() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());

    let test_message = BytesN::from_array(&env, &message_hash.into());

    let signature = bytes!(&env, 0x331fe75a821c982f9127538858900d87d3ec1f9f737338ad67cad133fa48feff48e6fa0c18abc62e42820f05943e47af3e9fbe306ce74d64094bdf1691ee53e01d);

    let _recovered_signer = ECDSA::recover(env, test_message, signature);
}

#[test]
fn to_eth_signed_message_hash_works() {
    let env = Env::default();

    let msg_str = "OpenZeppelin";
    let message_hash = Keccak256::digest(msg_str.as_bytes());
    let message_array = message_hash.into();

    let expected_hash = bytesn!(
        &env,
        0x7d768af957ef8cbf6219a37e743d5546d911dae3e46449d8a5810522db2ef65e
    );

    let message_hash = ECDSA::to_eth_signed_message_hash(env, message_array);

    assert!(message_hash == expected_hash);
}
