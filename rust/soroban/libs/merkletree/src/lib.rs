#![no_std]
use soroban_sdk::{
    assert_with_error, contract, contracterror, contractimpl, contracttype, symbol_short, vec,
    BytesN, Env, Symbol, Vec,
};
use tiny_keccak::{Hasher, Keccak};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MerkleTreeFull = 1,
    MerkleTreeInvalidVecSize = 2,
}

const TREE_DEPTH: usize = 32;
const MAX_LEAVES: u64 = u64::pow(2, TREE_DEPTH as u32) - 1;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleTree {
    pub branch: Vec<BytesN<32>>,
    pub count: u32,
}

impl MerkleTree {
    pub fn keccak256(items: Vec<[u8; 32]>) -> [u8; 32] {
        let mut hasher = Keccak::v256();
        let mut output: [u8; 32] = [0; 32];

        for item in items {
            hasher.update(&item);
        }

        hasher.finalize(&mut output);
        return output;
    }

    /**
     * @notice Inserts `_node` into merkle tree
     * @dev Reverts if tree is full
     * @param _node Element to insert into tree
     **/
    pub fn insert(&mut self, env: Env, mut _node: BytesN<32>) {
        assert_with_error!(
            &env,
            (self.count as u64) < MAX_LEAVES,
            Error::MerkleTreeFull
        );

        assert_with_error!(
            &env,
            self.branch.len() <= TREE_DEPTH as u32,
            Error::MerkleTreeInvalidVecSize
        );

        self.count += 1;
        let mut size = self.count;
        for i in 0..TREE_DEPTH as u32 {
            if (size & 1) == 1 {
                let item_pos = self.branch.get(i);

                if item_pos.is_none() {
                    self.branch.insert(i, _node);
                } else {
                    self.branch.set(i, _node);
                }
                return;
            }

            let leaf = self.branch.get(i).expect("Error to get leaf");

            let _vec = vec![&env, leaf.to_array(), _node.to_array()];

            let output = Self::keccak256(_vec);
            _node = BytesN::from_array(&env, &output);

            size /= 2;
        }
        // As the loop should always end prematurely with the `return` statement,
        // this code should be unreachable. We assert `false` just to be safe.
        assert!(false);
    }

    /**
     * @notice Calculates and returns`_tree`'s current root given array of zero
     * hashes
     * @param _zeroes Array of zero hashes
     * @return _current Calculated root of `_tree`
     **/
    fn root_with_ctx(&self, env: Env, _zeroes: Vec<BytesN<32>>) -> BytesN<32> {
        assert_with_error!(
            &env,
            self.branch.len() <= TREE_DEPTH as u32 && _zeroes.len() == TREE_DEPTH as u32,
            Error::MerkleTreeInvalidVecSize
        );

        let mut _current = BytesN::from_array(&env, &[0; 32]);
        let _index = self.count;

        for i in 0..TREE_DEPTH as u32 {
            let _ith_bit = (_index >> i) & 0x01;
            let _next = self
                .branch
                .get(i)
                .unwrap_or(BytesN::from_array(&env, &[0; 32]));
            if _ith_bit == 1 {
                let _vec = vec![&env, _next.clone().to_array(), _current.clone().to_array()];
                let value = Self::keccak256(_vec);
                _current = BytesN::from_array(&env, &value)
            } else {
                let hash = _zeroes.get_unchecked(i);
                let _vec = vec![&env, _current.clone().to_array(), hash.clone().to_array()];
                let value = Self::keccak256(_vec);
                _current = BytesN::from_array(&env, &value)
            }
        }
        return _current;
    }

    /// @notice Calculates and returns`_tree`'s current root
    pub fn root(&self, env: Env) -> BytesN<32> {
        let _zeroes = Self::zero_hashes(env.clone());
        return Self::root_with_ctx(&self, env.clone(), _zeroes);
    }

    /**
     * @notice Calculates and returns the merkle root for the given leaf
     * `_item`, a merkle branch, and the index of `_item` in the tree.
     * @param _item Merkle leaf
     * @param _branch Merkle proof
     * @param _index Index of `_item` in tree
     * @return Calculated merkle root
     **/
    pub fn branch_root(
        env: Env,
        _item: BytesN<32>,
        _branch: Vec<BytesN<32>>,
        _index: u64,
    ) -> BytesN<32> {
        let mut _current = _item;

        for i in 0..TREE_DEPTH as u32 {
            let _ith_bit = (_index >> i) & 0x01;
            let _next = _branch.get(i).unwrap_or(BytesN::from_array(&env, &[0; 32]));
            if _ith_bit == 1 {
                let _vec = vec![&env, _next.to_array(), _current.to_array()];
                let value = Self::keccak256(_vec);
                _current = BytesN::from_array(&env, &value)
            } else {
                let _vec = vec![&env, _current.to_array(), _next.to_array()];
                let value = Self::keccak256(_vec);
                _current = BytesN::from_array(&env, &value)
            }
        }
        return _current;
    }

    /// @notice Returns array of TREE_DEPTH zero hashes
    /// @return _zeroes Array of TREE_DEPTH zero hashes
    fn zero_hashes(env: Env) -> Vec<BytesN<32>> {
        let mut _zeroes = vec![&env];
        _zeroes.insert(0, BytesN::from_array(&env, &[0; 32]));
        _zeroes.insert(
            1,
            BytesN::from_array(
                &env,
                &[
                    173, 50, 40, 182, 118, 247, 211, 205, 66, 132, 165, 68, 63, 23, 241, 150, 43,
                    54, 228, 145, 179, 10, 64, 178, 64, 88, 73, 229, 151, 186, 95, 181,
                ],
            ),
        );
        _zeroes.insert(
            2,
            BytesN::from_array(
                &env,
                &[
                    180, 193, 25, 81, 149, 124, 111, 143, 100, 44, 74, 246, 28, 214, 178, 70, 64,
                    254, 198, 220, 127, 198, 7, 238, 130, 6, 169, 158, 146, 65, 13, 48,
                ],
            ),
        );
        _zeroes.insert(
            3,
            BytesN::from_array(
                &env,
                &[
                    33, 221, 185, 163, 86, 129, 92, 63, 172, 16, 38, 182, 222, 197, 223, 49, 36,
                    175, 186, 219, 72, 92, 155, 165, 163, 227, 57, 138, 4, 183, 186, 133,
                ],
            ),
        );
        _zeroes.insert(
            4,
            BytesN::from_array(
                &env,
                &[
                    229, 135, 105, 179, 42, 27, 234, 241, 234, 39, 55, 90, 68, 9, 90, 13, 31, 182,
                    100, 206, 45, 211, 88, 231, 252, 191, 183, 140, 38, 161, 147, 68,
                ],
            ),
        );
        _zeroes.insert(
            5,
            BytesN::from_array(
                &env,
                &[
                    14, 176, 30, 191, 201, 237, 39, 80, 12, 212, 223, 201, 121, 39, 45, 31, 9, 19,
                    204, 159, 102, 84, 13, 126, 128, 5, 129, 17, 9, 225, 207, 45,
                ],
            ),
        );
        _zeroes.insert(
            6,
            BytesN::from_array(
                &env,
                &[
                    136, 124, 34, 189, 135, 80, 211, 64, 22, 172, 60, 102, 181, 255, 16, 45, 172,
                    221, 115, 246, 176, 20, 231, 16, 181, 30, 128, 34, 175, 154, 25, 104,
                ],
            ),
        );
        _zeroes.insert(
            7,
            BytesN::from_array(
                &env,
                &[
                    255, 215, 1, 87, 228, 128, 99, 252, 51, 201, 122, 5, 15, 127, 100, 2, 51, 191,
                    100, 108, 201, 141, 149, 36, 198, 185, 43, 207, 58, 181, 111, 131,
                ],
            ),
        );
        _zeroes.insert(
            8,
            BytesN::from_array(
                &env,
                &[
                    152, 103, 204, 95, 127, 25, 107, 147, 186, 225, 226, 126, 99, 32, 116, 36, 69,
                    210, 144, 242, 38, 56, 39, 73, 139, 84, 254, 197, 57, 247, 86, 175,
                ],
            ),
        );
        _zeroes.insert(
            9,
            BytesN::from_array(
                &env,
                &[
                    206, 250, 212, 229, 8, 192, 152, 185, 167, 225, 216, 254, 177, 153, 85, 251, 2,
                    186, 150, 117, 88, 80, 120, 113, 9, 105, 211, 68, 15, 80, 84, 224,
                ],
            ),
        );
        _zeroes.insert(
            10,
            BytesN::from_array(
                &env,
                &[
                    249, 220, 62, 127, 224, 22, 224, 80, 239, 242, 96, 51, 79, 24, 165, 212, 254,
                    57, 29, 130, 9, 35, 25, 245, 150, 79, 46, 46, 183, 193, 195, 165,
                ],
            ),
        );
        _zeroes.insert(
            11,
            BytesN::from_array(
                &env,
                &[
                    248, 177, 58, 73, 226, 130, 246, 9, 195, 23, 168, 51, 251, 141, 151, 109, 17,
                    81, 124, 87, 29, 18, 33, 162, 101, 210, 90, 247, 120, 236, 248, 146,
                ],
            ),
        );
        _zeroes.insert(
            12,
            BytesN::from_array(
                &env,
                &[
                    52, 144, 198, 206, 235, 69, 10, 236, 220, 130, 226, 130, 147, 3, 29, 16, 199,
                    215, 59, 248, 94, 87, 191, 4, 26, 151, 54, 10, 162, 197, 217, 156,
                ],
            ),
        );
        _zeroes.insert(
            13,
            BytesN::from_array(
                &env,
                &[
                    193, 223, 130, 217, 196, 184, 116, 19, 234, 226, 239, 4, 143, 148, 180, 211,
                    85, 76, 234, 115, 217, 43, 15, 122, 249, 110, 2, 113, 198, 145, 226, 187,
                ],
            ),
        );
        _zeroes.insert(
            14,
            BytesN::from_array(
                &env,
                &[
                    92, 103, 173, 215, 198, 202, 243, 2, 37, 106, 222, 223, 122, 177, 20, 218, 10,
                    207, 232, 112, 212, 73, 163, 164, 137, 247, 129, 214, 89, 232, 190, 204,
                ],
            ),
        );
        _zeroes.insert(
            15,
            BytesN::from_array(
                &env,
                &[
                    218, 123, 206, 159, 78, 134, 24, 182, 189, 47, 65, 50, 206, 121, 140, 220, 122,
                    96, 231, 225, 70, 10, 114, 153, 227, 198, 52, 42, 87, 150, 38, 210,
                ],
            ),
        );
        _zeroes.insert(
            16,
            BytesN::from_array(
                &env,
                &[
                    39, 51, 229, 15, 82, 110, 194, 250, 25, 162, 43, 49, 232, 237, 80, 242, 60,
                    209, 253, 249, 76, 145, 84, 237, 58, 118, 9, 162, 241, 255, 152, 31,
                ],
            ),
        );
        _zeroes.insert(
            17,
            BytesN::from_array(
                &env,
                &[
                    225, 211, 181, 200, 7, 178, 129, 228, 104, 60, 198, 214, 49, 92, 249, 91, 154,
                    222, 134, 65, 222, 252, 179, 35, 114, 241, 193, 38, 227, 152, 239, 122,
                ],
            ),
        );
        _zeroes.insert(
            18,
            BytesN::from_array(
                &env,
                &[
                    90, 45, 206, 10, 138, 127, 104, 187, 116, 86, 15, 143, 113, 131, 124, 44, 46,
                    187, 203, 247, 255, 251, 66, 174, 24, 150, 241, 63, 124, 116, 121, 160,
                ],
            ),
        );
        _zeroes.insert(
            19,
            BytesN::from_array(
                &env,
                &[
                    180, 106, 40, 182, 245, 85, 64, 248, 148, 68, 246, 61, 224, 55, 142, 61, 18,
                    27, 224, 158, 6, 204, 157, 237, 28, 32, 230, 88, 118, 211, 106, 160,
                ],
            ),
        );
        _zeroes.insert(
            20,
            BytesN::from_array(
                &env,
                &[
                    198, 94, 150, 69, 100, 71, 134, 182, 32, 226, 221, 42, 214, 72, 221, 252, 191,
                    74, 126, 91, 26, 58, 78, 207, 231, 246, 70, 103, 163, 240, 183, 226,
                ],
            ),
        );
        _zeroes.insert(
            21,
            BytesN::from_array(
                &env,
                &[
                    244, 65, 133, 136, 237, 53, 162, 69, 140, 255, 235, 57, 185, 61, 38, 241, 141,
                    42, 177, 59, 220, 230, 174, 229, 142, 123, 153, 53, 158, 194, 223, 217,
                ],
            ),
        );
        _zeroes.insert(
            22,
            BytesN::from_array(
                &env,
                &[
                    90, 156, 22, 220, 0, 214, 239, 24, 183, 147, 58, 111, 141, 198, 92, 203, 85,
                    102, 113, 56, 119, 111, 125, 234, 16, 16, 112, 220, 135, 150, 227, 119,
                ],
            ),
        );
        _zeroes.insert(
            23,
            BytesN::from_array(
                &env,
                &[
                    77, 248, 79, 64, 174, 12, 130, 41, 208, 214, 6, 158, 92, 143, 57, 167, 194,
                    153, 103, 122, 9, 211, 103, 252, 123, 5, 227, 188, 56, 14, 230, 82,
                ],
            ),
        );
        _zeroes.insert(
            24,
            BytesN::from_array(
                &env,
                &[
                    205, 199, 37, 149, 247, 76, 123, 16, 67, 208, 225, 255, 186, 183, 52, 100, 140,
                    131, 141, 251, 5, 39, 217, 113, 182, 2, 188, 33, 108, 150, 25, 239,
                ],
            ),
        );
        _zeroes.insert(
            25,
            BytesN::from_array(
                &env,
                &[
                    10, 191, 90, 201, 116, 161, 237, 87, 244, 5, 10, 165, 16, 221, 156, 116, 245,
                    8, 39, 123, 57, 215, 151, 59, 178, 223, 204, 197, 238, 176, 97, 141,
                ],
            ),
        );
        _zeroes.insert(
            26,
            BytesN::from_array(
                &env,
                &[
                    184, 205, 116, 4, 111, 243, 55, 240, 167, 191, 44, 142, 3, 225, 15, 100, 44,
                    24, 134, 121, 141, 113, 128, 106, 177, 232, 136, 217, 229, 238, 135, 208,
                ],
            ),
        );
        _zeroes.insert(
            27,
            BytesN::from_array(
                &env,
                &[
                    131, 140, 86, 85, 203, 33, 198, 203, 131, 49, 59, 90, 99, 17, 117, 223, 244,
                    150, 55, 114, 204, 233, 16, 129, 136, 179, 74, 200, 124, 129, 196, 30,
                ],
            ),
        );
        _zeroes.insert(
            28,
            BytesN::from_array(
                &env,
                &[
                    102, 46, 228, 221, 45, 215, 178, 188, 112, 121, 97, 177, 230, 70, 196, 4, 118,
                    105, 220, 182, 88, 79, 13, 141, 119, 13, 175, 93, 126, 125, 235, 46,
                ],
            ),
        );
        _zeroes.insert(
            29,
            BytesN::from_array(
                &env,
                &[
                    56, 138, 178, 14, 37, 115, 209, 113, 168, 129, 8, 231, 157, 130, 14, 152, 242,
                    108, 11, 132, 170, 139, 47, 74, 164, 150, 141, 187, 129, 142, 163, 34,
                ],
            ),
        );
        _zeroes.insert(
            30,
            BytesN::from_array(
                &env,
                &[
                    147, 35, 124, 80, 186, 117, 238, 72, 95, 76, 34, 173, 242, 247, 65, 64, 11,
                    223, 141, 106, 156, 199, 223, 126, 202, 229, 118, 34, 22, 101, 215, 53,
                ],
            ),
        );
        _zeroes.insert(
            31,
            BytesN::from_array(
                &env,
                &[
                    132, 72, 129, 139, 180, 174, 69, 98, 132, 158, 148, 158, 23, 172, 22, 224, 190,
                    22, 104, 142, 21, 107, 92, 241, 94, 9, 140, 98, 124, 0, 86, 169,
                ],
            ),
        );

        return _zeroes;
    }
}

const TREE: Symbol = symbol_short!("TREE");

/**
 * This a basic helper contract used to assist with tests.
 */
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn get_tree(env: Env) -> MerkleTree {
        //let array = [BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32]),BytesN::from_array(&env, &[0;32])];
        return env.storage().instance().get(&TREE).unwrap_or(MerkleTree {
            branch: vec![&env],
            count: 0,
        });
    }

    pub fn insert(env: Env, node: BytesN<32>) -> MerkleTree {
        let mut tree = Self::get_tree(env.clone());

        tree.insert(env.clone(), node);

        // Save the tree.
        env.storage().instance().set(&TREE, &tree);

        return tree;
    }

    pub fn get_root(env: Env) -> BytesN<32> {
        let tree = Self::get_tree(env.clone());
        let root = tree.root(env.clone());
        return root;
    }
}

#[cfg(test)]
mod tests;
