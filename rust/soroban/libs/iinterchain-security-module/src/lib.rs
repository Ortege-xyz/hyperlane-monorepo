use soroban_sdk::{contracttype, Bytes, BytesN};

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
    fn module_type(&self) -> BytesN<1>;

    fn verify(&self, _metadata: Bytes, _message: Bytes) -> bool;
}

pub trait ISpecifiesInterchainSecurityModule {
    fn interchain_security_module() -> Box<dyn IInterchainSecurityModule>;
}
