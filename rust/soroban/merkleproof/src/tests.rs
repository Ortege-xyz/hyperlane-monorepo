use crate::{Contract, ContractClient};
use soroban_sdk::{vec, BytesN, Env, U256};
use tiny_keccak::{Hasher, Keccak};

#[test]
fn verify_works() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let proof = vec![
        &env,
        BytesN::from_array(
            &env,
            &[
                193, 103, 176, 227, 200, 34, 56, 244, 242, 209, 165, 10, 139, 58, 68, 249, 99, 17,
                215, 123, 20, 140, 48, 220, 14, 248, 99, 225, 160, 96, 220, 182,
            ],
        ),
        BytesN::from_array(
            &env,
            &[
                103, 54, 32, 115, 118, 117, 226, 117, 92, 232, 38, 154, 153, 144, 64, 34, 209, 93,
                168, 213, 132, 63, 90, 236, 32, 92, 210, 67, 255, 128, 36, 10,
            ],
        ),
    ];

    let root = BytesN::from_array(
        &env,
        &[
            76, 140, 227, 146, 111, 2, 147, 177, 210, 129, 234, 200, 29, 143, 119, 61, 194, 169,
            51, 62, 149, 30, 123, 25, 154, 222, 235, 199, 215, 162, 237, 102,
        ],
    );

    let mut hasher = Keccak::v256();
    let mut output: [u8; 32] = [0; 32];

    // to get he leaf we need to use keccak256 function two times, keccak256(keccak256(uint256(1)))
    let value = U256::from_u32(&env, 1);
    let value_byte = value.to_be_bytes();
    let value_byte_array: BytesN<32> = value_byte.try_into().expect("Error");

    hasher.update(&value_byte_array.to_array());
    hasher.finalize(&mut output);

    let mut final_leaf: [u8; 32] = [0; 32];
    hasher = Keccak::v256();
    hasher.update(&output);
    hasher.finalize(&mut final_leaf);

    let leaf = BytesN::from_array(&env, &final_leaf);

    let result = client.verify(&proof, &root, &leaf);
    assert!(result, "Proof check failed");
}

#[test]
fn verify_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let proof = vec![
        &env,
        BytesN::from_array(
            &env,
            &[
                093, 103, 176, 227, 200, 34, 56, 244, 242, 209, 165, 10, 139, 58, 68, 249, 99, 17,
                215, 123, 20, 140, 48, 220, 14, 248, 99, 225, 160, 96, 220, 182,
            ],
        ),
        BytesN::from_array(
            &env,
            &[
                10, 54, 32, 115, 118, 117, 226, 117, 92, 232, 38, 154, 153, 144, 64, 34, 209, 93,
                168, 213, 132, 63, 90, 236, 32, 92, 210, 67, 255, 128, 36, 10,
            ],
        ),
    ];

    let root = BytesN::from_array(
        &env,
        &[
            76, 140, 227, 146, 111, 2, 147, 177, 210, 129, 234, 200, 29, 143, 119, 61, 194, 169,
            51, 62, 149, 30, 123, 25, 154, 222, 235, 199, 215, 162, 237, 102,
        ],
    );

    let mut hasher = Keccak::v256();
    let mut output: [u8; 32] = [0; 32];

    let value = U256::from_u32(&env, 1);
    let value_byte = value.to_be_bytes();
    let value_byte_array: BytesN<32> = value_byte.try_into().expect("Error");

    hasher.update(&value_byte_array.to_array());
    hasher.finalize(&mut output);

    let mut final_leaf: [u8; 32] = [0; 32];
    hasher = Keccak::v256();
    hasher.update(&output);
    hasher.finalize(&mut final_leaf);

    let leaf = BytesN::from_array(&env, &final_leaf);

    let result = client.verify(&proof, &root, &leaf);
    assert!(result == false, "Proof check failed");
}

#[test]
fn multi_verify_works() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let root = BytesN::from_array(
        &env,
        &[
            76, 140, 227, 146, 111, 2, 147, 177, 210, 129, 234, 200, 29, 143, 119, 61, 194, 169,
            51, 62, 149, 30, 123, 25, 154, 222, 235, 199, 215, 162, 237, 102,
        ],
    );

    let proof = vec![
        &env,
        BytesN::from_array(
            &env,
            &[
                26, 176, 198, 148, 138, 39, 83, 73, 174, 69, 160, 106, 173, 102, 168, 189, 101,
                172, 24, 7, 70, 21, 213, 54, 118, 192, 155, 103, 128, 144, 153, 224,
            ],
        ),
        BytesN::from_array(
            &env,
            &[
                181, 217, 216, 148, 19, 58, 115, 10, 166, 81, 239, 98, 210, 107, 15, 250, 132, 98,
                51, 199, 65, 119, 165, 145, 164, 168, 150, 173, 253, 169, 125, 34,
            ],
        ),
    ];

    let proof_flags = vec![&env, false, false, true];
    let leaves = vec![
        &env,
        BytesN::from_array(
            &env,
            &[
                37, 132, 219, 74, 104, 170, 139, 23, 47, 112, 188, 4, 226, 231, 69, 65, 97, 124, 0,
                51, 116, 222, 110, 180, 178, 149, 232, 35, 229, 190, 171, 1,
            ], // 3, keccak256(keccak256(uint256(3)))
        ),
        BytesN::from_array(
            &env,
            &[
                193, 103, 176, 227, 200, 34, 56, 244, 242, 209, 165, 10, 139, 58, 68, 249, 99, 17,
                215, 123, 20, 140, 48, 220, 14, 248, 99, 225, 160, 96, 220, 182,
            ], // 4, keccak256(keccak256(uint256(4)))
        ),
    ];

    let result = client.multi_proof_verify(&proof, &proof_flags, &root, &leaves);
    assert!(result, "Proof check failed");
}

#[test]
fn multi_verify_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let root = BytesN::from_array(
        &env,
        &[
            76, 140, 227, 146, 111, 2, 147, 177, 210, 129, 234, 200, 29, 143, 119, 61, 194, 169,
            51, 62, 149, 30, 123, 25, 154, 222, 235, 199, 215, 162, 237, 102,
        ],
    );

    let proof = vec![
        &env,
        BytesN::from_array(
            &env,
            &[
                26, 176, 198, 148, 138, 39, 83, 73, 174, 69, 160, 106, 173, 102, 168, 189, 101,
                172, 24, 7, 70, 21, 213, 54, 118, 192, 155, 103, 128, 144, 153, 224,
            ],
        ),
        BytesN::from_array(
            &env,
            &[
                181, 217, 216, 148, 19, 58, 115, 10, 166, 81, 239, 98, 210, 107, 15, 250, 132, 98,
                51, 199, 65, 119, 165, 145, 164, 168, 150, 173, 253, 169, 125, 34,
            ],
        ),
    ];

    let proof_flags = vec![&env, false, false, true];
    let leaves = vec![
        &env,
        BytesN::from_array(
            &env,
            &[
                37, 132, 219, 74, 104, 170, 139, 23, 47, 112, 188, 4, 226, 231, 69, 65, 97, 124, 0,
                51, 116, 222, 110, 180, 178, 149, 232, 35, 229, 190, 171, 0,
            ],
        ),
        BytesN::from_array(
            &env,
            &[
                193, 103, 176, 227, 200, 34, 56, 244, 242, 209, 165, 10, 139, 58, 68, 249, 99, 17,
                215, 123, 20, 140, 48, 220, 14, 248, 99, 225, 160, 96, 220, 0,
            ],
        ),
    ];

    let result = client.multi_proof_verify(&proof, &proof_flags, &root, &leaves);
    assert!(result == false, "Proof check failed");
}
