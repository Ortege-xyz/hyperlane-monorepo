#![no_std]
use ownable::Ownable;
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    RemoteGasData(u32),
}

#[contracttype]
#[derive(Clone)]
pub struct RemoteGasDataConfig {
    remote_omain: u32,
    token_exchange_rate: u128,
    gas_price: u128,
}

#[contracttype]
#[derive(Clone)]
pub struct RemoteGasData {
    token_exchange_rate: u128,
    gas_price: u128,
}

#[contract]
pub struct StorageGasOracle;

#[contractimpl]
impl StorageGasOracle {
    pub fn init(env: Env, owner: Address) {
        Ownable::init(env.clone(), owner);
    }

    pub fn get_exchangerate_and_gasprice(env: Env, _destination_domain: u32) -> (u128, u128) {
        let _data: RemoteGasData = env
            .storage()
            .persistent()
            .get(&DataKey::RemoteGasData(_destination_domain))
            .expect("StorageGasOracle: error to get remote gas data.");

        return (_data.token_exchange_rate, _data.gas_price);
    }

    pub fn set_remote_gas_data_configs(
        env: Env,
        caller: Address,
        _configs: Vec<RemoteGasDataConfig>,
    ) {
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller);

        let _len = _configs.len();
        for i in 0.._len {
            Self::_set_remote_gas_data(env.clone(), _configs.get_unchecked(i));
        }
    }

    pub fn set_remote_gas_data(env: Env, caller: Address, _config: RemoteGasDataConfig) {
        caller.require_auth();
        Ownable::only_owner(env.clone(), caller);

        Self::_set_remote_gas_data(env, _config)
    }

    fn _set_remote_gas_data(env: Env, _config: RemoteGasDataConfig) {
        let _remote_gas_data = RemoteGasData {
            token_exchange_rate: _config.token_exchange_rate,
            gas_price: _config.gas_price,
        };

        env.storage().persistent().set(
            &DataKey::RemoteGasData(_config.remote_omain),
            &_remote_gas_data,
        );

        env.events().publish(
            (
                symbol_short!("REMOTE"),
                symbol_short!("GASDATA"),
                symbol_short!("SET"),
            ),
            (
                _config.remote_omain,
                _config.token_exchange_rate,
                _config.gas_price,
            ),
        );
    }
}
