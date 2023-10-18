use crate::{Contract, ContractClient};
use soroban_sdk::Env;

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

    let updated_initialized = client.get_initialized_version();
    assert_eq!(updated_initialized, 1)
}

//Multiple calls: Verifies that calling the initializer function multiple times results in an InvalidInitialization error.
// #[test]
// fn test_initializer_multiple_calls() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, Contract);
//     let client = ContractClient::new(&env, &contract_id);

//     let result = client.initializer();
//     // Assert successful initialization
//     assert_eq!(result, ());

//     let result1 = client.initializer();
//     assert_eq!(result1, ());

//     // let result = client.initializer();

//     // assert_eq!(client.initializer(), Err(Ok(ContractError::InvalidInitialization)));
// }

// //Reinitializing the contract without initializing it first
// #[test]
// fn test_reinitializer_initial_setup() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Reinitialize the contract to version 2 without initializing it first
//     initializable.reinitializer(&mut env, 2).unwrap();
//     assert_eq!(initializable.get_initialized_version(), 2);
// }

// //Reinitializing the contract to the same version
// #[test]
// fn test_reinitializer_already_initialized() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Initialize the contract to version 1
//     initializable.initializer(&mut env).unwrap();

//     // Try to reinitialize the contract to version 1, which should fail
//     assert_eq!(
//         initializable.reinitializer(&mut env, 1),
//         Err(ContractError::InvalidInitialization)
//     );
// }

// //Reinitializing the contract to a lower version
// #[test]
// fn test_reinitializer_already_reinitialized() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Initialize the contract to version 1
//     initializable.initializer(&mut env).unwrap();

//     // Reinitialize the contract to version 2
//     initializable.reinitializer(&mut env, 2).unwrap();

//     // Try to reinitialize the contract to version 2, which should fail
//     assert_eq!(
//         initializable.reinitializer(&mut env, 2),
//         Err(ContractError::InvalidInitialization)
//     );
// }

// //Reinitializing the contract to the maximum version
// #[test]
// fn test_reinitializer_max_version() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Initialize the contract to version 1
//     initializable.initializer(&mut env).unwrap();

//     // Reinitialize the contract to the maximum version
//     initializable.reinitializer(&mut env, u64::MAX).unwrap();

//     // Try to reinitialize the contract to any version, which should fail
//     assert_eq!(
//         initializable.reinitializer(&mut env, 1),
//         Err(ContractError::InvalidInitialization)
//     );
//     assert_eq!(
//         initializable.reinitializer(&mut env, 2),
//         Err(ContractError::InvalidInitialization)
//     );
//     assert_eq!(
//         initializable.reinitializer(&mut env, u64::MAX),
//         Err(ContractError::InvalidInitialization)
//     );
// }

// //function when the contract is initializing, which should succeed
// #[test]
// fn test_only_initializing_success() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Initialize the contract
//     initializable.initializer(&mut env).unwrap();

//     // Call the only_initializing function, which should succeed
//     initializable.only_initializing().unwrap();
// }

// //function when the contract is not initializing, which should fail
// #[test]
// fn test_only_initializing_failure() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Call the only_initializing function, which should fail
//     assert_eq!(
//         initializable.only_initializing(),
//         Err(ContractError::NotInitializing)
//     );
// }

// #[test]
// fn test_check_initializing() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Check initializing when not initializing, which should fail
//     assert_eq!(
//         initializable.check_initializing(),
//         Err(ContractError::NotInitializing)
//     );

//     // Initialize the contract
//     initializable.initializer(&mut env).unwrap();

//     // Check initializing when initializing, which should succeed
//     initializable.check_initializing().unwrap();
// }

// //Disabling initializers when the contract is not already disabled, which should succeed and prevent any further reinitialization.
// #[test]
// fn test_disable_initializers_success() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Initialize the contract
//     initializable.initializer(&mut env).unwrap();

//     // Disable initializers
//     initializable.disable_initializers(&mut env).unwrap();

//     // Try to reinitialize the contract, which should fail
//     assert_eq!(
//         initializable.reinitializer(&mut env, 2),
//         Err(ContractError::InvalidInitialization)
//     );
// }

// //Disabling initializers when the contract is already disabled, which should succeed but do nothing since the contract is already disabled.
// #[test]
// fn test_disable_initializers_already_disabled() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Initialize the contract to the maximum version
//     initializable.reinitializer(&mut env, u64::MAX).unwrap();

//     // Try to disable initializers, which should succeed but do nothing
//     initializable.disable_initializers(&mut env).unwrap();

//     // Try to reinitialize the contract, which should still fail
//     assert_eq!(
//         initializable.reinitializer(&mut env, 2),
//         Err(ContractError::InvalidInitialization)
//     );
// }

// #[test]
// fn test_get_initialized_version() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Check the initialized version before initialization
//     assert_eq!(initializable.get_initialized_version(), 0);

//     // Initialize the contract to version 2
//     initializable.reinitializer(&mut env, 2).unwrap();

//     // Check the initialized version after initialization
//     assert_eq!(initializable.get_initialized_version(), 2);
// }

// #[test]
// fn test_is_initializing() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Check if initializing before initialization
//     assert!(!initializable.is_initializing());

//     // Initialize the contract
//     initializable.initializer(&mut env).unwrap();

//     // Check if initializing after initialization
//     assert!(initializable.is_initializing());
// }

// #[test]
// fn test_get_initializable_storage() {
//     let mut env = Env::default();
//     let mut initializable = Initializable::new();

//     // Get the initializable storage
//     let storage = initializable.get_initializable_storage();

//     // Check the initialized version
//     assert_eq!(storage.initialized, 0);

//     // Check if initializing
//     assert!(!storage.initializing);
// }
