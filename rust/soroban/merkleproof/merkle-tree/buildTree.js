import { StandardMerkleTree } from '@openzeppelin/merkle-tree';
import fs from 'fs';

// (1)
const values = [[1], [2], [3], [4]];

// (2)
const tree = StandardMerkleTree.of(values, ['uint256']);

// (3)
console.log('Merkle Root:', tree.root);

// (4)
fs.writeFileSync('tree.json', JSON.stringify(tree.dump()));
