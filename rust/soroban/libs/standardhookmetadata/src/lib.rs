use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, U256};

const VARIANT_OFFSET: u32 = 0;
const MSG_VALUE_OFFSET: u32 = 2;
const GAS_LIMIT_OFFSET: u32 = 34;
const REFUND_ADDRESS_OFFSET: u32 = 66;
const MIN_METADATA_LENGTH: u32 = 86;

pub const VARIANT: u16 = 1;

pub struct StandardHookMetadata;

impl StandardHookMetadata {
    /**
     * @notice Returns the variant of the metadata.
     * @param _metadata ABI encoded global hook metadata.
     * @return variant of the metadata as uint8.
     */
    pub fn variant(_metadata: Bytes) -> u16 {
        if _metadata.len() < VARIANT_OFFSET + 2 {
            return 0;
        }

        let bytes = _metadata.slice(VARIANT_OFFSET..(VARIANT_OFFSET + 2));
        let mut slice = [0u8; 2];
        bytes.copy_into_slice(&mut slice);
        let variant = u16::from_be_bytes(slice);
        return variant;
    }

    /**
     * @notice Returns the specified value for the message.
     * @param _metadata ABI encoded global hook metadata.
     * @param _default Default fallback value.
     * @return Value for the message as uint256.
     */
    pub fn msg_value(env: Env, _metadata: Bytes, _default: U256) -> U256 {
        if _metadata.len() < MSG_VALUE_OFFSET + 32 {
            return _default;
        }

        let bytes = _metadata.slice(MSG_VALUE_OFFSET..(MSG_VALUE_OFFSET + 2));
        let msg_value = U256::from_be_bytes(&env, &bytes);
        return msg_value;
    }

    /**
     * @notice Returns the specified gas limit for the message.
     * @param _metadata ABI encoded global hook metadata.
     * @param _default Default fallback gas limit.
     * @return Gas limit for the message as uint256.
     */
    pub fn gas_limit(env: Env, _metadata: Bytes, _default: U256) -> U256 {
        if _metadata.len() < GAS_LIMIT_OFFSET + 32 {
            return _default;
        }

        let bytes = _metadata.slice(GAS_LIMIT_OFFSET..(GAS_LIMIT_OFFSET + 2));
        let gas_limit = U256::from_be_bytes(&env, &bytes);
        return gas_limit;
    }

    /**
     * @notice Returns the specified refund address for the message.
     * @param _metadata ABI encoded global hook metadata.
     * @param _default Default fallback refund address.
     * @return Refund address for the message as address.
     */
    pub fn refund_address(env: Env, _metadata: Bytes, _default: Address) -> Address {
        if _metadata.len() < REFUND_ADDRESS_OFFSET + 20 {
            return _default;
        }

        let mut address_slice = [0u8; 32];

        let address_bytes: BytesN<20> = _metadata
            .slice(REFUND_ADDRESS_OFFSET..(REFUND_ADDRESS_OFFSET + 20))
            .try_into()
            .expect("StandardHookMetadata: error to get refund address.");
        let address_array = address_bytes.to_array();

        address_slice[12..].copy_from_slice(&address_array);

        return Address::from_contract_id(&BytesN::from_array(&env, &address_slice));
    }

    /**
     * @notice Returns the specified refund address for the message.
     * @param _metadata ABI encoded global hook metadata.
     * @return Refund address for the message as address.
     */
    pub fn get_custom_metadata(_metadata: Bytes) -> Bytes {
        if _metadata.len() < MIN_METADATA_LENGTH {
            return _metadata.slice(0..0);
        };

        return _metadata.slice(MIN_METADATA_LENGTH..);
    }

    /**
     * @notice Formats the specified gas limit and refund address into global hook metadata.
     * @param _msgValue msg.value for the message.
     * @param _gasLimit Gas limit for the message.
     * @param _refundAddress Refund address for the message.
     * @param _customMetadata Additional metadata to include in the global hook metadata.
     * @return ABI encoded global hook metadata.
     */
    pub fn format_metadata(
        env: Env,
        _msg_value: U256,
        _gas_limit: U256,
        _refund_address: Address,
        _custom_metadata: Bytes,
    ) -> Bytes {
        let mut bytes = Bytes::from_array(&env, &[VARIANT as u8]);
        bytes.append(&_msg_value.to_be_bytes());
        bytes.append(&_gas_limit.to_be_bytes());
        bytes.append(&Bytes::from_array(
            &env,
            &_refund_address.contract_id().to_array(),
        ));
        bytes.append(&_custom_metadata);

        return bytes;
    }

    /**
     * @notice Formats the specified gas limit and refund address into global hook metadata.
     * @param _msgValue msg.value for the message.
     * @return ABI encoded global hook metadata.
     */
    pub fn format_metadata_msg_value(env: Env, _msg_value: U256, _caller: Address) -> Bytes {
        return Self::format_metadata(
            env.clone(),
            _msg_value,
            U256::from_u32(&env, 0),
            _caller,
            Bytes::from_array(&env, &[]),
        );
    }

    /**
     * @notice Formats the specified gas limit and refund address into global hook metadata.
     * @param _gasLimit Gas limit for the message.
     * @param _refundAddress Refund address for the message.
     * @return ABI encoded global hook metadata.
     */
    pub fn format_metadata_gas_address(env: Env, _gas_limit: U256, _refund_address: Address) -> Bytes{
        return Self::format_metadata(
            env.clone(),
            U256::from_u32(&env, 0),
            _gas_limit,
            _refund_address,
            Bytes::from_array(&env, &[]),
        );
    }
}
