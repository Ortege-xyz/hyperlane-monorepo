use crate::{Contract, ContractClient, ZERO};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

// This Function will test if the owner is the expected
#[test]
fn owner_address_correct() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let user2 = Address::random(&env);

    client.init(&user);

    let owner = client.owner();

    assert_eq!(owner, user);
    assert_ne!(owner, user2);
}

// This function will test if the owner increased the count
#[test]
fn invalid_owner_increase_count() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    client.init(&user);

    let count = client.increment(&user);

    assert_eq!(count, 1);
}

// This function will test if an invalid owner throw the error
#[test]
#[should_panic]
fn owner_increase_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let user2 = Address::random(&env);
    client.init(&user);

    client.increment(&user2);
}

// This function will test if the ownership has been transferred
#[test]
fn transfer_owner_ship() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let user2 = Address::random(&env);
    client.init(&user);

    client.transfer_owner_ship(&user, &user2);

    let owner = client.owner();

    assert!(owner == user2);
}

// This function will test if the ownership has been transferred
#[test]
#[should_panic]
fn transfer_owner_ship_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let user2 = Address::random(&env);
    client.init(&user);

    client.transfer_owner_ship(&user2, &user2);
}

// This function will test if new owner is 0 address
#[test]
fn renounce_ownership() {
    let env = Env::default();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    client.init(&user);

    client.mock_all_auths().renounce_ownership(&user);

    let owner = client.owner();

    let zero_address = Address::from_contract_id(&BytesN::from_array(&env, &ZERO));

    assert!(owner == zero_address);
}

#[test]
#[should_panic]
fn renounce_ownership_fails() {
    let env = Env::default();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::random(&env);
    let user2 = Address::random(&env);
    client.init(&user);

    client.mock_all_auths().renounce_ownership(&user2);
}
