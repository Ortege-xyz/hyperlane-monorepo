use soroban_sdk::{Bytes, BytesN};

pub trait IInterchainSecurityModule {
    fn handle(_origin: u32, _sender: BytesN<32>, _message: Bytes);
}