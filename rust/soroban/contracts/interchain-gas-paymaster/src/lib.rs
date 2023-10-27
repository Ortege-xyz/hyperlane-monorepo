#![no_std]
use message::Message;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Bytes, BytesN, Env,
    FromVal, String, Symbol, U256,
};
use standardhookmetadata::StandardHookMetadata;
use storage_gas_oracle::StorageGasOracleClient;

const BENEFICIARY: Symbol = symbol_short!("BENEFICIA");
const GASCONFIG: Symbol = symbol_short!("GASCONFIG");
const GAS: Symbol = symbol_short!("GAS");
const PAYMENT: Symbol = symbol_short!("PAYMENT");
const SET: Symbol = symbol_short!("SET");

const TOKEN_EXCHANGE_RATE_SCALE: u128 = 10000000000; //1e10
const DEFAULT_GAS_USAGE: u32 = 50000;

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
struct GasParam {
    remote_domain: u32,
    config: DomainGasConfig,
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
     * @notice Deposits msg.value as a payment for the relaying of a message
     * to its destination chain.
     * @dev Overpayment will result in a refund of native tokens to the _refundAddress.
     * Callers should be aware that this may present reentrancy issues.
     * @param _messageId The ID of the message to pay for.
     * @param _destinationDomain The domain of the message's destination chain.
     * @param _gasLimit The amount of destination gas to pay for.
     * @param _refundAddress The address to refund any overpayment to.
     */
    pub fn pay_for_gas(
        env: Env,
        _message_id: BytesN<32>,
        _destination_domain: u32,
        _gas_limit: U256,
        _refund_address: Address,
        _msg_value: U256,
        _caller: Address,
    ) {
        let _required_payment =
            Self::quote_gas_payment(env.clone(), _destination_domain.clone(), _gas_limit.clone());

        assert!(
            _msg_value >= _required_payment,
            "IGP: insufficient interchain gas payment"
        );

        // TODO:fix this code after the pr below go live
        let xlm_address = String::from_slice(
            &env,
            &"CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT",
        ); // future net /address
        let xlm = token::Client::new(&env, &Address::from_val(&env, &xlm_address.to_val()));

        let mut _over_payment = _msg_value.sub(&_required_payment);

        let current_address = env.current_contract_address();

        if _over_payment > U256::from_u32(&env, 0) {
            // TODO ADD the ASSERT after this pr go to live https://github.com/stellar/rs-soroban-sdk/pull/1112
            //assert!(_refund_address != ZERO_ADD)
            let mut continue_transfer = true;
            while continue_transfer {
                let amount: i128 = {
                    let mut amount = 0i128;
                    let max = i128::MAX;
                    if _over_payment
                        > U256::from_be_bytes(&env, &Bytes::from_array(&env, &max.to_be_bytes()))
                    {
                        _over_payment = _over_payment.sub(&U256::from_be_bytes(
                            &env,
                            &Bytes::from_array(&env, &max.to_be_bytes()),
                        ));
                        amount = max;
                    } else {
                        let mut slice = [0u8; 16];
                        _over_payment.to_be_bytes().copy_into_slice(&mut slice);
                        amount = i128::from_be_bytes(slice);
                        continue_transfer = false;
                    }
                    amount
                };

                xlm.transfer(&current_address, &_caller, &amount);
            }
        }

        let mut continue_transfer = true;
        while continue_transfer {
            let amount: i128 = {
                let mut amount = 0i128;
                let max = i128::MAX;
                if _over_payment
                    > U256::from_be_bytes(&env, &Bytes::from_array(&env, &max.to_be_bytes()))
                {
                    _over_payment = _over_payment.sub(&U256::from_be_bytes(
                        &env,
                        &Bytes::from_array(&env, &max.to_be_bytes()),
                    ));
                    amount = max;
                } else {
                    let mut slice = [0u8; 16];
                    _over_payment.to_be_bytes().copy_into_slice(&mut slice);
                    amount = i128::from_be_bytes(slice);
                    continue_transfer = false;
                }
                amount
            };

            xlm.transfer_from(&current_address, &_caller, &current_address, &amount)
        }

        env.events().publish(
            (GAS, PAYMENT),
            (
                _message_id,
                _destination_domain,
                _gas_limit,
                _required_payment,
            ),
        );
    }

    /**
     * @notice Quotes the amount of native tokens to pay for interchain gas.
     * @param _destinationDomain The domain of the message's destination chain.
     * @param _gasLimit The amount of destination gas to pay for.
     * @return The amount of native tokens required to pay for interchain gas.
     */
    pub fn quote_gas_payment(env: Env, _destination_domain: u32, _gas_limit: U256) -> U256 {
        // Get the gas data for the destination domain.
        let (_token_exchangerate, _gas_price) =
            Self::get_exchangerate_and_gasprice(env.clone(), _destination_domain);

        // The total cost quoted in destination chain's native token
        let _destination_gas_cost = _gas_limit.mul(&U256::from_be_bytes(
            &env,
            &Bytes::from_array(&env, &_gas_price.to_be_bytes()),
        ));

        // Convert to the local native token.
        let amount = _destination_gas_cost
            .mul(&U256::from_be_bytes(
                &env,
                &Bytes::from_array(&env, &_gas_price.to_be_bytes()),
            ))
            .div(&U256::from_be_bytes(
                &env,
                &Bytes::from_array(&env, &TOKEN_EXCHANGE_RATE_SCALE.to_be_bytes()),
            ));

        return amount;
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

    /**
     * @notice Returns the stored destinationGasOverhead added to the _gasLimit.
     * @dev If there is no stored destinationGasOverhead, 0 is used. This is useful in the case
     *      the ISM deployer wants to subsidize the overhead gas cost. Then, can specify the gas oracle
     *      they want to use with the destination domain, but set the overhead to 0.
     * @param _destinationDomain The domain of the message's destination chain.
     * @param _gasLimit The amount of destination gas to pay for. This is only for application gas usage as
     *      the gas usage for the mailbox and the ISM is already accounted in the DomainGasConfig.gasOverhead
     */
    pub fn destination_gas_limit(env: Env, _destination_domain: u32, _gas_limit: U256) -> U256 {
        let _destination_gas_config: DomainGasConfig = env
            .storage()
            .persistent()
            .get(&DataKey::DestinationGasConfigs(_destination_domain))
            .expect("Configured IGP doesn't support domain ");

        let gas_overhead = _destination_gas_config.gas_overhead;

        return _gas_limit.add(&U256::from_be_bytes(
            &env,
            &Bytes::from_array(&env, &gas_overhead.to_be_bytes()),
        ));
    }

    fn _post_dispatch(
        env: Env,
        metadata: Bytes,
        message: Bytes,
        _msg_value: U256,
        _caller: Address,
    ) {
        let _destination_domain = Message::destination(message.clone());
        Self::pay_for_gas(
            env.clone(),
            Message::id(env.clone(), message.clone()),
            _destination_domain,
            Self::destination_gas_limit(
                env.clone(),
                _destination_domain,
                StandardHookMetadata::gas_limit(
                    env.clone(),
                    metadata.clone(),
                    U256::from_u32(&env, DEFAULT_GAS_USAGE),
                ),
            ),
            StandardHookMetadata::refund_address(
                env.clone(),
                metadata.clone(),
                Message::sender_address(message.clone()),
            ),
            _msg_value,
            _caller,
        )
    }

    fn _quote_dispatch(env: Env, metadata: Bytes, message: Bytes) -> U256 {
        return Self::quote_gas_payment(
            env.clone(),
            Message::destination(message.clone()),
            Self::destination_gas_limit(
                env.clone(),
                Message::destination(message.clone()),
                StandardHookMetadata::gas_limit(
                    env.clone(),
                    metadata,
                    U256::from_u32(&env, DEFAULT_GAS_USAGE),
                ),
            ),
        );
    }

    /**
     * @notice Sets the beneficiary.
     * @param _beneficiary The new beneficiary.
     */
    fn _set_beneficiary(env: Env, _beneficiary: Address) {
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
