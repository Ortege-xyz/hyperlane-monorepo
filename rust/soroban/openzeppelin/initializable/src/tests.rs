use crate::{Contract, ContractClient};
use soroban_sdk::{log, Env};

// This Function will test before initialize
#[test]
fn before_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let init_initialized = client.get_initialized_version();
    let init_initializing = client.is_initializing();

    assert_eq!(init_initialized, 0);
    assert_eq!(init_initializing, false);
}

//Tests the initial setup scenario where the contract is not initialized and not initializing.
#[test]
fn test_initializer_initial_setup() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let init_initialized = client.get_initialized_version();
    let init_initializing = client.is_initializing();

    assert_eq!(init_initialized, 0);
    assert_eq!(init_initializing, false);

    client.initializer();

    // Retrieve the updated initialized value from the contract
    let updated_initialized = client.get_initialized_version();
    let updated_initializing = client.is_initializing();
    log!(&env, "====init.storage.initialized{}", updated_initialized);

    assert_eq!(updated_initialized, 1);

    assert_eq!(updated_initializing, false);
}