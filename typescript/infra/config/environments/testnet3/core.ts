import { ChainMap, CoreConfig } from '@ortege/sdk';
import { objMap } from '@ortege/utils';

import { aggregationIsm } from '../../aggregationIsm';
import { Contexts } from '../../contexts';

import { owners } from './owners';

export const core: ChainMap<CoreConfig> = objMap(owners, (local, owner) => {
  const defaultIsm = aggregationIsm('testnet3', local, Contexts.Hyperlane);
  return {
    owner,
    defaultIsm,
  };
});
