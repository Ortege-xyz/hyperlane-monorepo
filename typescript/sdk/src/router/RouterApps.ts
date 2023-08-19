import type { BigNumber } from 'ethers';

import { GasRouter, Router } from '@ortege/core';
import { Address, objMap, promiseObjAll } from '@ortege/utils';

import { HyperlaneApp } from '../app/HyperlaneApp';
import { HyperlaneContracts, HyperlaneFactories } from '../contracts/types';
import { ChainMap, ChainName } from '../types';

export { Router } from '@ortege/core';

export abstract class RouterApp<
  Factories extends HyperlaneFactories,
> extends HyperlaneApp<Factories> {
  abstract router(contracts: HyperlaneContracts<Factories>): Router;

  getSecurityModules(): Promise<ChainMap<Address>> {
    return promiseObjAll(
      objMap(this.chainMap, (_, contracts) =>
        this.router(contracts).interchainSecurityModule(),
      ),
    );
  }

  getOwners(): Promise<ChainMap<Address>> {
    return promiseObjAll(
      objMap(this.chainMap, (_, contracts) => this.router(contracts).owner()),
    );
  }
}

export abstract class GasRouterApp<
  Factories extends HyperlaneFactories,
  R extends GasRouter,
> extends RouterApp<Factories> {
  abstract router(contracts: HyperlaneContracts<Factories>): R;

  async quoteGasPayment(
    origin: ChainName,
    destination: ChainName,
  ): Promise<BigNumber> {
    return this.getContracts(origin).router.quoteGasPayment(
      this.multiProvider.getDomainId(destination),
    );
  }
}
