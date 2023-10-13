use crate::Initializable;
use soroban_sdk::{Env};
// This Function will test if the owner is the expected
#[test]
fn before_initialize() {
    let env = Env::default();
    let mut set = Initializable::new(env.clone());

    assert_eq!(set.storage.initialized, 0);
    assert_eq!(set.storage.initializing, false);
}
