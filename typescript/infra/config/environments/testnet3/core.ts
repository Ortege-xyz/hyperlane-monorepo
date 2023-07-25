import {
  AggregationIsmConfig,
  ChainMap,
  CoreConfig,
  objMap,
} from '@ortege/sdk';

import { Contexts } from '../../contexts';

import { aggregationIsm } from './aggregationIsm';
import { owners } from './owners';

export const core: ChainMap<CoreConfig> = objMap(owners, (local, owner) => {
  const defaultIsm: AggregationIsmConfig = aggregationIsm(
    local,
    Contexts.Hyperlane,
  );
  return {
    owner,
    defaultIsm,
  };
});
