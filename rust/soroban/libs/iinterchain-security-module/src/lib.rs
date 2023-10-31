use soroban_sdk::{contracttype, Bytes, BytesN, Env};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Types {
    UNUSED,
    ROUTING,
    AGGREGATION,
    LEGACYMULTISIG,
    MERKLEROOTMULTISIG,
    MESSAGEIDMULTISIG,
    NULL, // used with relayer carrying no metadata
    CCIPREAD,
}

pub trait IInterchainSecurityModule {
    fn module_type(env: Env) -> BytesN<1>;

    fn verify(env: Env, _metadata: Bytes, _message: Bytes) -> bool;
}

// pub trait ISpecifiesInterchainSecurityModule {
//     fn interchain_security_module() -> Box<dyn IInterchainSecurityModule>;
// }
pub mod iinterchain_security_stub {

    use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

    use crate::IInterchainSecurityModule;
    #[contract]
    pub struct Dummy;

    #[contractimpl]
    #[allow(dead_code)]
    impl IInterchainSecurityModule for Dummy {
        fn verify(_env: Env, _metadata: Bytes, _message: Bytes) -> bool {
            // Nothing
            return true;
        }

        fn module_type(env: Env) -> BytesN<1> {
            let one = BytesN::from_array(&env, &[0]);
            return one;
        }
    }
}
