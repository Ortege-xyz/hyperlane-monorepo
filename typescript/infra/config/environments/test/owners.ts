import { ChainMap } from '@ortege/sdk';
import { Address } from '@ortege/utils';

import { chainNames } from './chains';

// Owner is hardhat account 0
const OWNER_ADDRESS = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
export const owners: ChainMap<Address> = Object.fromEntries(
  chainNames.map((chain) => [chain, OWNER_ADDRESS]),
);
