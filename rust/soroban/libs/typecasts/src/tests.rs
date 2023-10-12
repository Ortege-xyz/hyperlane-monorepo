use crate::{Contract, ContractClient};
use soroban_sdk::{Address, BytesN, Env};

#[test]
fn test_bytes32_to_address() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let vec32: [u8; 32] = [0; 32];

    let zero_address = Address::from_contract_id(&BytesN::from_array(&env, &[0; 32]));

    assert_eq!(
        client.bytes32_to_address(&BytesN::from_array(&env, &vec32)),
        zero_address
    );
}
