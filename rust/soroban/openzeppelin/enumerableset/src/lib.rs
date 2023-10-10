use soroban_sdk::{contracttype, BytesN, Env, Map, Vec};

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumerableSet {
    positions: Map<BytesN<32>, u32>,
    values: Vec<BytesN<32>>,
}

impl EnumerableSet {
    pub fn new(env: Env) -> Self {
        EnumerableSet {
            positions: Map::new(&env),
            values: Vec::new(&env),
        }
    }

    pub fn add(&mut self, value: BytesN<32>) -> bool {
        if !self.positions.contains_key(value.clone()) {
            let index = self.values.len() as u32;
            self.positions.set(value.clone(), index);
            self.values.push_back(value);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, value: BytesN<32>) -> bool {
        if let Some(index) = self.positions.get(value.clone()) {
            let last_index = (self.values.len() - 1) as u32;

            if index != last_index {
                let last_value = self.values.get_unchecked(last_index);
                self.values.set(index, last_value.clone());
                self.positions.set(last_value, index); // Update the position of the swapped value
            }

            self.values.pop_back();
            self.positions.remove(value);
            true
        } else {
            false
        }
    }

    // Returns true if the value is in the set. O(1)
    pub fn contains(&self, value: BytesN<32>) -> bool {
        self.positions.contains_key(value)
    }

    pub fn at(&self, index: u32) -> Option<BytesN<32>> {
        self.values.get(index)
    }

    pub fn values(&self) -> &Vec<BytesN<32>> {
        &self.values
    }

    pub fn len(&self) -> u32 {
        self.values.len()
    }
}

#[cfg(test)]
mod tests;
