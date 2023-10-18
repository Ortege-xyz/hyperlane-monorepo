use iinterchain_security_module::IInterchainSecurityModule;
use soroban_sdk::{Address, Bytes, BytesN};

pub trait IMailbox {
    fn localDomain(&self) -> u32;

    fn delivered(&self, _messageId: BytesN<32>) -> bool;

    fn defaultIsm(&self) -> Box<dyn IInterchainSecurityModule>;

    fn dispatch(
        &self,
        destination_domain: u32,
        recipient_address: BytesN<32>,
        message_body: Bytes,
    ) -> BytesN<32>;

    fn process(&self, _metadata: Bytes, _message: Bytes);

    fn count(&self) -> u32;

    fn root(&self) -> BytesN<32>;

    fn latestCheckpoint(&self) -> (BytesN<32>, u32);

    fn recipientIsm(&self, _recipient: Address) -> Box<dyn IInterchainSecurityModule>;
}
