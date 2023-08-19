import { InterchainQueryRouter__factory } from '@ortege/core';

import { proxiedFactories } from '../../router/types';

export const interchainQueryFactories = {
  interchainQueryRouter: new InterchainQueryRouter__factory(),
  ...proxiedFactories,
};

export type InterchainQueryFactories = typeof interchainQueryFactories;
