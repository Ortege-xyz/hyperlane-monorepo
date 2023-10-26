use crate::Message;
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env};

const BODY: &str = "hello world";

const ORIGIN: u32 = 10;
const NONCE: u32 = 0;
const VERSION: u32 = 1;
const DESTINATION: u32 = 15;

#[test]
fn version_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());

    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _version = message.version;

    assert!(VERSION == _version);
}

#[test]
fn nonce_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _nonce = message.nonce;

    assert!(NONCE == _nonce);
}

#[test]
fn origin_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _origin_domain = message.origin();

    assert!(ORIGIN == _origin_domain);
}

#[test]
fn sender_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _sender = message.sender();

    assert!(sender == _sender);
}
#[test]
fn destination_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _destination_domain = message.destination();

    assert!(DESTINATION == _destination_domain);
}

#[test]
fn recipient_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _recipient = message.recipient();

    assert!(recipient == _recipient);
}

#[test]
fn body_is_valid() {
    let env = Env::default();

    let body_bytes = Bytes::from_slice(&env, BODY.as_bytes());
    let sender = Address::random(&env);
    let recipient = Address::random(&env);

    let message = Message::new(
        VERSION,
        NONCE,
        ORIGIN,
        &sender,
        DESTINATION,
        &recipient,
        &body_bytes,
    );

    let _body = message.body();

    assert!(body_bytes.eq(&_body));
}
