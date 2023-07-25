import {
  InterchainQueryRouter__factory,
  ProxyAdmin__factory,
} from '@ortege/core';

export const interchainQueryFactories = {
  interchainQueryRouter: new InterchainQueryRouter__factory(),
  proxyAdmin: new ProxyAdmin__factory(),
};

export type InterchainQueryFactories = typeof interchainQueryFactories;
