import {
  CircleBridgeAdapter__factory,
  LiquidityLayerRouter__factory,
  PortalAdapter__factory,
} from '@ortege/core';

import { proxiedFactories } from '../../router/types';

export const liquidityLayerFactories = {
  circleBridgeAdapter: new CircleBridgeAdapter__factory(),
  portalAdapter: new PortalAdapter__factory(),
  liquidityLayerRouter: new LiquidityLayerRouter__factory(),
  ...proxiedFactories,
};

export type LiquidityLayerFactories = typeof liquidityLayerFactories;
