use soroban_sdk::{Address, Env, contract, contractimpl};

pub struct InterchainGasPaymaster;

struct DomainGasConfig {
    gasOracle: Address,
    gasOverhead: u128,
}

impl InterchainGasPaymaster {
    pub fn initialize(env: Env) {

    }

    pub fn claim(env: Env) -> u32 {
        return env.storage().persistent().get(&BLOCK_NUMBER).unwrap_or(0);
    }

    fn _set_destination_gas_config(_remote_domain: u32, _gas_oracle: Address, _gas_overhead: u128{
        
    }
}
