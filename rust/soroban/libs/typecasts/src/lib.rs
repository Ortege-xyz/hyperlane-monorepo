#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Alignment-preserving cast: bytes32 to address
    pub fn bytes32_to_address(buf: BytesN<32>) -> Address {
        Address::from_contract_id(&buf)
    }
}

#[cfg(test)]
mod tests;
