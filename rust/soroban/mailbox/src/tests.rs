use crate::{Contract, ContractClient, MerkleTree};
use soroban_sdk::{vec, BytesN, Env, Vec, U256};

#[test]
fn initialize_sets_default_values() {
    let env = Env::default();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    
    let owner = Address::from_contract_id(&BytesN::from_array(&env, &[1; 20]));
    let default_ism = Address::from_contract_id(&BytesN::from_array(&env, &[2; 20]));

    Mailbox::initialize(env.clone(), owner, default_ism);
    assert_eq!(
        env.storage().instance().get(&DEFAULT_ISM).unwrap(),
        &default_ism
    );
    assert_eq!(env.storage().instance().get(&COUNT).unwrap(), &0);
    assert_eq!(env.storage().instance().get(&PAUSED).unwrap(), &false);
}
