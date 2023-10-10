#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, Symbol,
};

pub struct Ownable;

const OWNER: Symbol = symbol_short!("OWNER");

pub const ZERO: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[contracttype]
struct OwnershipTransferred {
    new_owner: Address,
    old_owner: Address,
}

impl Ownable {
    pub fn init(env: Env, owner: Address) {
        let _owner = Self::owner(env.clone());
        let _zero_address = Address::from_contract_id(&BytesN::from_array(&env, &ZERO));

        if _owner != _zero_address {
            panic!("Ownable: Already initialized")
        }

        env.storage().instance().set(&OWNER, &owner);
    }

    pub fn only_owner(env: Env, caller: Address) {
        let owner = Self::owner(env.clone());

        assert!(owner == caller, "Ownable: Unauthorized Account")
    }

    pub fn owner(env: Env) -> Address {
        return env
            .storage()
            .instance()
            .get(&OWNER)
            .unwrap_or(Address::from_contract_id(&BytesN::from_array(&env, &ZERO)));
    }

    pub fn renounce_ownership(env: Env, caller: Address) {
        caller.require_auth();
        Self::only_owner(env.clone(), caller);

        let new_owner = Address::from_contract_id(&BytesN::from_array(&env, &ZERO));

        Self::_transfer_owner_ship(env.clone(), new_owner);
    }

    pub fn transfer_owner_ship(env: Env, caller: Address, new_owner: Address) {
        caller.require_auth();
        Self::only_owner(env.clone(), caller);

        assert!(
            !new_owner.eq(&Address::from_contract_id(&BytesN::from_array(&env, &ZERO))),
            "Ownable: Invalid Owner"
        );

        Self::_transfer_owner_ship(env.clone(), new_owner);
    }

    fn _transfer_owner_ship(env: Env, new_owner: Address) {
        let old_owner = Self::owner(env.clone());

        env.storage().instance().set(&OWNER, &new_owner);

        env.events().publish(
            (OWNER, "transferred", old_owner.clone()),
            OwnershipTransferred {
                new_owner: new_owner.clone(),
                old_owner: old_owner.clone(),
            },
        );
    }
}

//This is only a test contract
#[contract]
pub struct Contract;

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contractimpl]
impl Contract {
    pub fn init(env: Env, owner: Address) {
        Ownable::init(env.clone(), owner);
    }

    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, caller: Address) -> u32 {
        Ownable::only_owner(env.clone(), caller);
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// get_count returns the current value of the counter.
    pub fn get_count(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    // Ownable functions
    pub fn owner(env: Env) -> Address {
        return Ownable::owner(env);
    }

    pub fn renounce_ownership(env: Env, caller: Address) {
        Ownable::renounce_ownership(env, caller);
    }

    pub fn transfer_owner_ship(env: Env, caller: Address, new_owner: Address) {
        Ownable::transfer_owner_ship(env, caller, new_owner);
    }
}

#[cfg(test)]
mod tests;
