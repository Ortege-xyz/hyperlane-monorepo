#![no_std]
use address_lib::AddressLib;
use iinterchain_security_module::IInterchainSecurityModule;
use imessage_recipient::IMessageRecipient;
use merkletree::MerkleTree;
use message::Message;
use ownable::Ownable;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Bytes, BytesN, Env, Symbol,
};
use versioned::Versioned;

const MAX_MESSAGE_BODY_BYTES: u32 = 2 * (1 << 10);

const TREE: Symbol = symbol_short!("MAILTREE");
const LOCAL_DOMAIN: Symbol = symbol_short!("DOMAIN");
const DEFAULT_ISM: Symbol = symbol_short!("ISM");
const DELIVERED: Symbol = symbol_short!("DELIVERED");
const PAUSED: Symbol = symbol_short!("PAUSED");

// Define your Mailbox struct
#[contract]
pub struct Mailbox;

#[contracttype]
pub enum DataKey {
    Delivered(BytesN<32>),
}

// Implement the Mailbox struct
#[contractimpl]
impl Mailbox {
    pub fn get_tree(env: Env) -> MerkleTree {
        //let array = [BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32])];
        return env.storage().instance().get(&TREE).unwrap_or(MerkleTree {
            branch: vec![&env],
            count: 0,
        });
    }

    fn address_to_bytes32(address: Address) -> BytesN<32> {
        // Create a new BytesN<32> with all elements initialized to zero
        let mut bytes32 = BytesN::<32>::default();

        // Copy the address bytes into the lower 20 bytes of the BytesN<32>
        bytes32[..20].copy_from_slice(&address.0);

        bytes32
    }

    pub fn initialize(env: Env, owner: Address, default_ism: Address) {
        // Ownable::init(env.clone(), owner);
        Self::get_tree(env);
        env.storage().instance().set(&PAUSED, &false);
        env.storage().instance().set(&DEFAULT_ISM, &default_ism);
        Self::set_default_ism(env, default_ism);
    }

    // Sets the default ISM for the Mailbox.
    /**
     * @notice Sets the default ISM for the Mailbox.
     * @param _module The new default ISM. Must be a contract.
     */
    pub fn set_default_ism(env: Env, module: Address) {
        Ownable::only_owner(env.clone(), module);

        assert!(AddressLib::is_contract(module).unwrap(), "!contract");
        default_ism = IInterchainSecurityModule::From(module);
        env.storage().instance().set(&DEFAULT_ISM, &module);

        env.events()
            .publish::<(Symbol,), Address>((symbol_short!("ISM").into(),), module);
    }

    pub fn dispatch(
        env: Env,
        destination_domain: u32,
        sender_address: Address,
        recipient_address: BytesN<32>,
        message_body: Bytes,
    ) -> BytesN<32> {
        sender_address.require_auth();

        assert!(
            message_body.len() <= MAX_MESSAGE_BODY_BYTES,
            "Message body exceeds maximum allowed size"
        );

        let mut tree = Self::get_tree(env.clone());

        let sender_byte_n32: BytesN<32> = Self::address_to_bytes32(sender_address);

        let message = Message::new(
            Versioned::VERSION,
            tree.count,
            env.storage().instance().get(&LOCAL_DOMAIN).unwrap_or(0),
            &sender_byte_n32,
            destination_domain,
            &recipient_address,
            &message_body,
        );

        let id = message.id(env);
        tree.insert(env, id);

        env.events()
            .publish::<(Symbol,), (Address, u32, BytesN<32>, Bytes)>(
                (symbol_short!("DISPATCH").into(),),
                (
                    sender_address,
                    destination_domain,
                    recipient_address,
                    message,
                ),
            );

        env.events()
            .publish::<(Symbol,), BytesN<32>>((symbol_short!("DISID").into(),), id);

        id
    }

    pub fn process(env: Env, metadata: Bytes, message: Message) {
        // Check that the message was intended for this mailbox.
        assert!(
            message.version() == Versioned::VERSION,
            "Invalid message version"
        );
        assert!(
            message.destination() == env.storage().instance().get(&LOCAL_DOMAIN).unwrap_or(0),
            "Invalid message destination"
        );

        // Check that the message hasn't already been delivered.
        let id = message.id(env);

        // contract up well for adding other types of data to be stored.
        let key = DataKey::Delivered(id.clone());

        // Get the current value for the key.
        let mut check: bool = env.storage().persistent().get(&key).unwrap_or(false);

        // Assert that the message has not been delivered yet.
        assert!(!check, "Message has already been delivered");

        if !check {
            // Save the value.
            env.storage().persistent().set(&key, &true);
        }

        // Verify the message via the ISM.
        let ism: dyn IInterchainSecurityModule = Self::recipientIsm(env, message.recipient());
        assert!(ism.verify(metadata, message), "ISM verification failed");

        // Deliver the message to the recipient.
        let origin = message.origin();
        let sender = message.sender();
        let recipient = message.recipient();
        IMessageRecipient::handle(origin, sender, message.body())?;

        env.events()
            .publish::<(Symbol,), (u32, BytesN<32>, Address)>(
                (symbol_short!("PROCESS").into(),),
                (origin, sender, recipient),
            );

        env.events()
            .publish::<(Symbol,), BytesN<32>>((symbol_short!("PROCESSID").into(),), id);
    }

    pub fn root(env: Env) -> BytesN<32> {
        let tree = Self::get_tree(env.clone());

        tree.root(env.clone())
    }

    pub fn count(env: Env) -> u32 {
        let tree = Self::get_tree(env.clone());
        tree.count
    }

    pub fn latest_checkpoint(env: Env) -> (BytesN<32>, u32) {
        (Self::root(env), Self::count(env) - 1)
    }

    //Returns the ISM to use for the recipient, defaulting to the default ISM if none is specified.
    pub fn recipientIsm(env: Env, recipient: Address) -> IInterchainSecurityModule {
        let zero_address = Address::from_contract_id(&BytesN::from_array(&env, &[0; 32]));

        // If a specified ISM exists, return it; otherwise, return the default ISM
        if recipient != zero_address {
            recipient
        } else {
            env.storage().instance().get(&DEFAULT_ISM).unwrap()
        }
    }
}

#[cfg(test)]
mod tests;
