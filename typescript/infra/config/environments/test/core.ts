import {
  ChainMap,
  CoreConfig,
  ModuleType,
  RoutingIsmConfig,
  objMap,
} from '@ortege/sdk';

import { multisigIsm } from './multisigIsm';
import { owners } from './owners';

export const core: ChainMap<CoreConfig> = objMap(owners, (local, owner) => {
  const defaultIsm: RoutingIsmConfig = {
    type: ModuleType.ROUTING,
    owner,
    domains: Object.fromEntries(
      Object.entries(multisigIsm).filter(([chain]) => chain !== local),
    ),
  };

  return {
    owner,
    defaultIsm,
  };
});
