#![no_std]
use soroban_sdk::xdr::ToXdr;
use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Message {
    version: u32,
    nonce: u32,
    origin_domain: u32,
    sender: Address,
    destination_domain: u32,
    recipient: Address,
    body: Bytes,
}

impl Message {
    pub fn new(
        _version: u32,
        _nonce: u32,
        _origin_domain: u32,
        _sender: &Address,
        _destination_domain: u32,
        _recipient: &Address,
        _message_body: &Bytes,
    ) -> Self {
        Self {
            version: _version,
            nonce: _nonce,
            origin_domain: _origin_domain,
            sender: _sender.clone(),
            destination_domain: _destination_domain,
            recipient: _recipient.clone(),
            body: _message_body.clone(),
        }
    }

    pub fn id(&self, env: Env) -> BytesN<32> {
        let message: Message = self.clone();
        let serialized_message = message.to_xdr(&env);
        let hashed = env.crypto().keccak256(&serialized_message);

        return hashed;
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn nonce(&self) -> u32 {
        self.nonce
    }

    pub fn origin(&self) -> u32 {
        self.origin_domain
    }

    pub fn sender(&self) -> Address {
        self.sender.clone()
    }

    // pub fn sender_address(&self) -> Address {
    //     Address::from_contract_id(&self.sender)
    // }

    pub fn destination(&self) -> u32 {
        self.destination_domain
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    // pub fn recipient_address(&self) -> Address {
    //     Address::from_contract_id(&self.recipient)
    // }

    pub fn body(&self) -> Bytes {
        self.body.clone()
    }
}

#[cfg(test)]
mod tests;
