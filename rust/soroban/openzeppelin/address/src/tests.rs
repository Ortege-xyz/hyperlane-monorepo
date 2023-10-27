use crate::AddressLib;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
pub fn check_contract_address() {
    let env = Env::default();
    let address = Address::random(&env);
    assert!(AddressLib::is_contract(address).unwrap());
}
