use crate::Message;
use soroban_sdk::{Bytes, BytesN, Env};

const BODY: &str = "hello world";
const SENDER_ARRAY: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const RECIPIENT_ADDRESS: [u8; 32] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const ORIGIN: u32 = 10;
const NONCE: u32 = 0;
const VERSION: u8 = 1u8;
const DESTINATION: u32 = 15;

#[test]
fn message_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let mut bytes_array = Bytes::from_array(&env, &[VERSION]);
    bytes_array.extend_from_array(&NONCE.to_be_bytes());
    bytes_array.extend_from_array(&ORIGIN.to_be_bytes());
    bytes_array.extend_from_array(&SENDER_ARRAY);
    bytes_array.extend_from_array(&DESTINATION.to_be_bytes());
    bytes_array.extend_from_array(&RECIPIENT_ADDRESS);
    bytes_array.append(&body_bytes);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    assert!(message == bytes_array);
}

#[test]
fn version_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _version = Message::version(message);

    assert!(VERSION == _version);
}

#[test]
fn nonce_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _nonce = Message::nonce(message);

    assert!(NONCE == _nonce);
}

#[test]
fn origin_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _origin_domain = Message::origin(message);

    assert!(ORIGIN == _origin_domain);
}

#[test]
fn sender_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _sender = Message::sender(message);

    assert!(sender == _sender);
}
#[test]
fn destination_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _destination_domain = Message::destination(message);

    assert!(DESTINATION == _destination_domain);
}

#[test]
fn recipient_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _recipient = Message::recipient(message);

    assert!(recipient == _recipient);
}

#[test]
fn body_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = BytesN::from_array(&env, &SENDER_ARRAY);
    let recipient = BytesN::from_array(&env, &RECIPIENT_ADDRESS);

    let message = Message::format_message(
        env,
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _body = Message::body(message);

    assert!(body_bytes.eq(&_body));
}
