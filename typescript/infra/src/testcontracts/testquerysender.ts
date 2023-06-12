import { TestQuerySender__factory } from '@ortege/core';
import {
  ChainName,
  HyperlaneDeployer,
  HyperlaneIgp,
  MultiProvider,
} from '@ortege/sdk';

export const factories = {
  TestQuerySender: new TestQuerySender__factory(),
};

type TestQuerySenderConfig = { queryRouterAddress: string };

export class TestQuerySenderDeployer extends HyperlaneDeployer<
  TestQuerySenderConfig,
  typeof factories
> {
  constructor(multiProvider: MultiProvider, protected igp: HyperlaneIgp) {
    super(multiProvider, factories);
  }
  async deployContracts(chain: ChainName, config: TestQuerySenderConfig) {
    const TestQuerySender = await this.deployContract(
      chain,
      'TestQuerySender',
      [],
      [
        config.queryRouterAddress,
        this.igp.getContracts(chain).interchainGasPaymaster.address,
      ],
    );
    return {
      TestQuerySender,
    };
  }
}
