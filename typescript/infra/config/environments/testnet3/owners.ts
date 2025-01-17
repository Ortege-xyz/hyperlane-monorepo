import { ChainMap } from '@ortege/sdk';
import { Address } from '@ortege/utils';

import { chainNames } from './chains';

const DEPLOYER_ADDRESS = '0xfaD1C94469700833717Fa8a3017278BC1cA8031C';

export const owners: ChainMap<Address> = Object.fromEntries(
  chainNames.map((chain) => [chain, DEPLOYER_ADDRESS]),
);
