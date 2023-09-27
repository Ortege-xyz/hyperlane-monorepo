//#![no_std]
use soroban_sdk::{Address, Bytes, BytesN, Env};

use tiny_keccak::{Hasher, Keccak};

const VERSION_OFFSET: u32 = 0;
const NONCE_OFFSET: u32 = 1;
const ORIGIN_OFFSET: u32 = 5;
const SENDER_OFFSET: u32 = 9;
const DESTINATION_OFFSET: u32 = 41;
const RECIPIENT_OFFSET: u32 = 45;
const BODY_OFFSET: u32 = 77;

pub struct Message;

pub const ZERO: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

impl Message {
    pub fn format_message(
        env: Env,
        _version: u8,
        _nonce: u32,
        _origin_domain: u32,
        _sender: &BytesN<32>,
        _destination_domain: u32,
        _recipient: &BytesN<32>,
        _message_body: &Bytes,
    ) -> Bytes {
        let bytes_nonce = Bytes::from_array(&env, &_nonce.to_be_bytes());
        let bytes_origin_domain = Bytes::from_array(&env, &_origin_domain.to_be_bytes());
        let bytes_destination_domain = Bytes::from_array(&env, &_destination_domain.to_be_bytes());
        let bytes_sender = Bytes::from_array(&env, &_sender.to_array());
        let bytes_recipient = Bytes::from_array(&env, &_recipient.to_array());

        let mut message = Bytes::from_array(&env, &[_version]);
        message.append(&bytes_nonce);
        message.append(&bytes_origin_domain);
        message.append(&bytes_sender);
        message.append(&bytes_destination_domain);
        message.append(&bytes_recipient);
        message.append(_message_body);

        return message;
    }

    pub fn id(env: Env, _message: Bytes) -> BytesN<32> {
        let mut hasher = Keccak::v256();
        let mut output: [u8; 32] = [0; 32];

        let mut slice = [0u8];
        let slice = &mut slice[..];
        _message.copy_into_slice(slice);

        hasher.update(slice);
        hasher.finalize(&mut output);

        return BytesN::from_array(&env, &output);
    }

    pub fn version(_message: Bytes) -> u8 {
        return _message.get(VERSION_OFFSET).expect("Error to get version");
    }

    pub fn nonce(_message: Bytes) -> u32 {
        let bytes = _message.slice(NONCE_OFFSET..ORIGIN_OFFSET);
        let mut slice: [u8; 4] = [0; 4];
        bytes.copy_into_slice(&mut slice);
        let nonce = u32::from_be_bytes(slice);
        return nonce;
    }

    pub fn origin(_message: Bytes) -> u32 {
        let bytes = _message.slice(ORIGIN_OFFSET..SENDER_OFFSET);
        let mut slice: [u8; 4] = [0; 4];
        bytes.copy_into_slice(&mut slice);
        let origin = u32::from_be_bytes(slice);
        return origin;
    }

    pub fn sender(_message: Bytes) -> BytesN<32> {
        let bytes = _message.slice(SENDER_OFFSET..DESTINATION_OFFSET);
        let sender = bytes.try_into().expect("Error to decode the sender");
        return sender;
    }

    pub fn sender_address(_message: Bytes) -> Address {
        let bytes = Self::sender(_message);
        let sender = Address::from_contract_id(&bytes);
        return sender;
    }

    pub fn destination(_message: Bytes) -> u32 {
        let bytes = _message.slice(DESTINATION_OFFSET..RECIPIENT_OFFSET);
        let mut slice: [u8; 4] = [0; 4];
        bytes.copy_into_slice(&mut slice);
        let destination = u32::from_be_bytes(slice);
        return destination;
    }

    pub fn recipient(_message: Bytes) -> BytesN<32> {
        let bytes = _message.slice(RECIPIENT_OFFSET..BODY_OFFSET);
        let recipient = bytes.try_into().expect("Error to decode the destination");
        return recipient;
    }

    pub fn recipient_address(_message: Bytes) -> Address {
        let bytes = Self::recipient(_message);
        let recipient = Address::from_contract_id(&bytes);
        return recipient;
    }

    pub fn body(_message: Bytes) -> Bytes {
        let body = _message.slice(BODY_OFFSET..);
        return body;
    }
}

#[cfg(test)]
mod tests;
