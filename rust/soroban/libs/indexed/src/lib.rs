use soroban_sdk::{symbol_short, Env, Symbol};

const BLOCK_NUMBER: Symbol = symbol_short!("BNUMBER");

pub struct Indexed;

impl Indexed {
    pub fn init(env: Env) {
        let _deployed_block = Self::deployed_block(env.clone());

        if _deployed_block != 0 {
            panic!("Indexed: Already initialized")
        }

        let deployed_block = env.ledger().sequence();
        env.storage()
            .persistent()
            .set(&BLOCK_NUMBER, &deployed_block);
    }

    pub fn deployed_block(env: Env) -> u32 {
        return env.storage().persistent().get(&BLOCK_NUMBER).unwrap_or(0);
    }
}
