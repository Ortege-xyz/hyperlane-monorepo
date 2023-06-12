import { HyperlaneConnectionClient, ProxyAdmin__factory } from '@ortege/core';
import type { types } from '@ortege/utils';

import { HyperlaneFactories } from '../contracts';
import { CheckerViolation } from '../deploy/types';

export type OwnableConfig = {
  owner: types.Address;
};

export type RouterConfig = ConnectionClientConfig & OwnableConfig;

export type GasConfig = {
  gas: number;
};

export type GasRouterConfig = RouterConfig & GasConfig;

export type ProxiedFactories = HyperlaneFactories & {
  proxyAdmin: ProxyAdmin__factory;
};

export type ConnectionClientConfig = {
  mailbox: types.Address;
  interchainGasPaymaster: types.Address;
  interchainSecurityModule?: types.Address;
};

export enum ConnectionClientViolationType {
  InterchainSecurityModule = 'ConnectionClientIsm',
  Mailbox = 'ConnectionClientMailbox',
  InterchainGasPaymaster = 'ConnectionClientIgp',
}

export interface ConnectionClientViolation extends CheckerViolation {
  type: ConnectionClientViolationType;
  contract: HyperlaneConnectionClient;
  actual: string;
  expected: string;
}
