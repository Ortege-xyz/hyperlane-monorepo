use crate::{Mailbox, MailboxClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn initialize_sets_default_values() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Mailbox);
    let client = MailboxClient::new(&env, &contract_id);

    // let owner = Address::from_contract_id(&BytesN::from_array(&env, &[1; 32]));
    // let default_ism = Address::from_contract_id(&BytesN::from_array(&env, &[1; 32]));

    let owner = Address::random(&env);
    let default_ism = Address::random(&env);

    client.initialize(&owner, &default_ism);

    let count = client.count();

    assert_eq!(count, 0);
}
