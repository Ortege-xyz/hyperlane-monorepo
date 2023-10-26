#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, Symbol,
};
use storage_gas_oracle::StorageGasOracleClient;

const BENEFICIARY: Symbol = symbol_short!("BENEFICIA");
const GASCONFIG: Symbol = symbol_short!("GASCONFIG");
const SET: Symbol = symbol_short!("SET");

#[contract]
pub struct InterchainGasPaymaster;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    DestinationGasConfigs(u32),
    Beneficiary,
}

#[contracttype]
#[derive(Clone)]
struct DomainGasConfig {
    gas_oracle: Address,
    gas_overhead: u128,
}

#[contractimpl]
impl InterchainGasPaymaster {
    pub fn initialize(env: Env) {}

    pub fn claim(env: Env) -> u32 {
        return env
            .storage()
            .persistent()
            .get(&DataKey::Beneficiary)
            .unwrap_or(0);
    }
    /**
     * @notice Gets the token exchange rate and gas price from the configured gas oracle
     * for a given destination domain.
     * @param _destinationDomain The destination domain.
     * @return tokenExchangeRate The exchange rate of the remote native token quoted in the local native token.
     * @return gasPrice The gas price on the remote chain.
     */
    pub fn get_exchangerate_and_gasprice(env: Env, _destination_domain: u32) -> (u128, u128) {
        let _destination_gas_config: DomainGasConfig = env
            .storage()
            .persistent()
            .get(&DataKey::DestinationGasConfigs(_destination_domain))
            .expect("Configured IGP doesn't support domain ");
        let _gas_oracle = _destination_gas_config.gas_oracle;

        let storage_client = StorageGasOracleClient::new(&env, &_gas_oracle);
        return storage_client.get_exchangerate_and_gasprice(&_destination_domain);
    }

    fn _quote_dispatch(metadata: Bytes, message: Bytes) {}

    /**
     * @notice Sets the beneficiary.
     * @param _beneficiary The new beneficiary.
     */
    fn _setBeneficiary(env: Env, _beneficiary: Address) {
        env.storage()
            .instance()
            .set(&DataKey::Beneficiary, &_beneficiary);

        env.events().publish((BENEFICIARY, SET), _beneficiary)
    }

    /**
     * @notice Sets the gas oracle and destination gas overhead for a remote domain.
     * @param _remoteDomain The remote domain.
     * @param _gasOracle The gas oracle.
     * @param _gasOverhead The destination gas overhead.
     */
    fn _set_destination_gas_config(
        env: Env,
        _remote_domain: u32,
        _gas_oracle: Address,
        _gas_overhead: u128,
    ) {
        env.storage().persistent().set(
            &DataKey::DestinationGasConfigs(_remote_domain),
            &DomainGasConfig {
                gas_oracle: _gas_oracle.clone(),
                gas_overhead: _gas_overhead,
            },
        );

        env.events().publish(
            (GASCONFIG, SET),
            (_remote_domain, _gas_oracle, _gas_overhead),
        );
    }
}
