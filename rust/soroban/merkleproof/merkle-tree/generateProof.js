import { StandardMerkleTree } from '@openzeppelin/merkle-tree';
import fs from 'fs';

import { hexStringToByteArray } from './utils.js';

// (1)
const tree = StandardMerkleTree.load(
  JSON.parse(fs.readFileSync('tree.json', 'utf8')),
);

console.log('Tree root: ', tree.root);

// (2)
for (const [i, v] of tree.entries()) {
  // (3)
  const proof = tree.getProof(i);
  console.log('Value: ', v);
  console.log('Proof hex: ', proof);
  console.log(
    'Proof array: ',
    proof.map((value) => hexStringToByteArray(value.slice(2))),
  );
  console.log('Index: ', i);
}
