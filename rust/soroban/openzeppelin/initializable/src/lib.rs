#![no_std]
use core::primitive::u64;
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Env, Symbol};

/**
 * @dev Storage of the initializable contract.
 *
 * It's implemented on a custom ERC-7201 namespace to reduce the risk of storage collisions
 * when using with upgradeable contracts.
 *
 * @custom:storage-location erc7201:openzeppelin.storage.Initializable
 */

#[contracttype]
#[derive(Clone)]
pub struct InitializableStorage {
    initialized: u64,
    initializing: bool,
}

impl InitializableStorage {
    fn new() -> Self {
        Self {
            initialized: 0,
            initializing: false,
        }
    }
}

// keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.Initializable")) - 1)) & ~bytes32(uint256(0xff))
pub const INITIALIZABLE_STORAGE: [u8; 32] = [
    0xf0, 0xc5, 0x7e, 0x16, 0x84, 0x0d, 0xf0, 0x40, 0xf1, 0x50, 0x88, 0xdc, 0x2f, 0x81, 0xfe, 0x39,
    0x1c, 0x39, 0x23, 0xbe, 0xc7, 0x3e, 0x23, 0xa9, 0x66, 0x2e, 0xfc, 0x9c, 0x22, 0x9c, 0x6a, 0x00,
];

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    InvalidInitialization = 1, // @dev The contract is already initialized.
    NotInitializing = 2,       //  @dev The contract is not initializing.
}

const COUNTER: Symbol = symbol_short!("COUNTER");

//TODO: check which storage actullay need env storage or stuct storage
#[derive(Clone)]
pub struct Initializable {
    storage: InitializableStorage,
}

impl Initializable {
    pub fn new() -> Self {
        Self {
            storage: InitializableStorage::new(),
        }
    }

    // A protected initializer function that can be invoked at most once.
    pub fn initializer(&mut self, env: Env) -> Result<(), ContractError> {
        let is_top_level_call = !self.storage.initializing;
        let initialized = self.storage.initialized;

        // Allowed calls:
        // - initialSetup: the contract is not in the initializing state and no previous version was
        //                 initialized
        // - construction: the contract is initialized at version 1 (no reininitialization) and the
        //                 current contract is just being deployed
        let is_initial_setup = initialized < 1 && is_top_level_call;
        let construction = initialized == 1; //TODO: need to check and add code length but can not found which one is code

        if !is_initial_setup && !construction {
            return Err(ContractError::InvalidInitialization);
        }

        // assert_with_error!(
        //     &env,
        //     !is_initial_setup && !construction,
        //     ContractError::InvalidInitialization
        // );

        self.storage.initialized = 1;
        if is_top_level_call {
            self.storage.initializing = true;
        }

        self.storage.initializing = false;

        env.events().publish((COUNTER, symbol_short!("initial")), 1);

        Ok(())
    }

    // A protected reinitializer function that can be invoked at most once.
    pub fn reinitializer(&mut self, env: &Env, version: u64) -> Result<(), ContractError> {
        let initializing = self.storage.initializing;
        let initialized = self.storage.initialized;

        // assert_with_error!(
        //     &env,
        //     initializing || initialized >= version,
        //     ContractError::InvalidInitialization
        // );

        if initializing || initialized >= version {
            return Err(ContractError::InvalidInitialization);
        }

        self.storage.initialized = version;
        self.storage.initializing = true;

        self.storage.initializing = false;

        env.events()
            .publish((COUNTER, symbol_short!("initial")), version);

        Ok(())
    }

    // Modifier to protect an initialization function.
    pub fn only_initializing(&self) -> Result<(), ContractError> {
        self.check_initializing()
    }

    // Reverts if the contract is not in an initializing state.
    pub fn check_initializing(&self) -> Result<(), ContractError> {
        // assert_with_error!(
        //     &env,
        //     !self.is_initializing(),
        //     ContractError::NotInitializing
        // );

        if !self.is_initializing() {
            return Err(ContractError::NotInitializing);
        }
        Ok(())
    }

    // Locks the contract, preventing any future reinitialization.
    pub fn disable_initializers(&mut self, env: Env) -> Result<(), ContractError> {
        // assert_with_error!(
        //     &env,
        //     self.storage.initializing,
        //     ContractError::InvalidInitialization
        // );

        if self.storage.initializing {
            return Err(ContractError::InvalidInitialization);
        }

        if self.storage.initialized != u64::MAX {
            self.storage.initialized = u64::MAX;
            env.events()
                .publish((COUNTER, symbol_short!("initial")), u64::MAX);
        }

        Ok(())
    }

    // Returns the highest version that has been initialized.
    pub fn get_initialized_version(&self) -> u64 {
        self.storage.initialized
    }

    // Returns true if the contract is currently initializing.
    pub fn is_initializing(&self) -> bool {
        self.storage.initializing
    }

    // Returns a pointer to the storage namespace.
    fn get_initializable_storage(&mut self) -> &mut InitializableStorage {
        &mut self.storage
    }
}

/**
 * This a basic helper contract used to assist with tests.
 */
#[contract]
pub struct Contract;

const STORAGE: Symbol = symbol_short!("STORAGE");

#[contractimpl]
impl Contract {
    pub fn get_storage(env: Env) -> InitializableStorage {
        return env
            .storage()
            .instance()
            .get(&STORAGE)
            .unwrap_or(InitializableStorage {
                initialized: 0,
                initializing: false,
            });
    }

    pub fn initializer(env: Env) -> Result<(), ContractError> {
        let mut init = Initializable::new();

        // Call initializer once and handle the result
        let result = init.initializer(env.clone());

        match result {
            Ok(()) => {
                // Initialization successful, get the updated storage
                let storage_struct = init.get_initializable_storage();
                env.storage().instance().set(&STORAGE, storage_struct);

                return Ok(());
            }
            Err(err) => {
                // Handle other errors accordingly
                return Err(err);
            }
        }
    }

    // Reverts if the contract is not in an initializing state.
    pub fn check_initializing() -> Result<(), ContractError> {
        let init = Initializable::new();

        // Call initializer once and handle the result
        return init.check_initializing();
    }

    pub fn disable_initializers(env: Env) -> Result<(), ContractError> {
        let mut init = Initializable::new();

        // Call initializer once and handle the result
        let result = init.disable_initializers(env.clone());

        match result {
            Ok(()) => {
                // Initialization successful, get the updated storage as a mutable reference
                let storage_struct = init.get_initializable_storage();

                // if storage_struct.initialized != u64::MAX {
                //     storage_struct.initialized = u64::MAX;

                env.storage().instance().set(&STORAGE, storage_struct);
                //}
            }
            Err(err) => {
                // Handle other errors accordingly
                return Err(err);
            }
        }

        Ok(())
    }

    pub fn get_initialized_version(env: Env) -> u64 {
        let storage = Self::get_storage(env.clone());
        storage.initialized
    }

    pub fn is_initializing(env: Env) -> bool {
        let storage = Self::get_storage(env.clone());
        storage.initializing
    }
}

#[cfg(test)]
mod tests;
