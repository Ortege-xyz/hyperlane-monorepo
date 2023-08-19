import {
  TestRecipient__factory,
  TestTokenRecipient__factory,
} from '@ortege/core';
import { ChainName, HyperlaneDeployer, MultiProvider } from '@ortege/sdk';

export const TEST_RECIPIENT_DEPLOYER_FACTORIES = {
  TestRecipient: new TestRecipient__factory(),
  TestTokenRecipient: new TestTokenRecipient__factory(),
};

export class TestRecipientDeployer extends HyperlaneDeployer<
  never,
  typeof TEST_RECIPIENT_DEPLOYER_FACTORIES
> {
  constructor(multiProvider: MultiProvider) {
    super(multiProvider, TEST_RECIPIENT_DEPLOYER_FACTORIES);
  }

  async deployContracts(chain: ChainName) {
    const TestRecipient = await this.deployContract(chain, 'TestRecipient', []);
    const TestTokenRecipient = await this.deployContract(
      chain,
      'TestTokenRecipient',
      [],
    );
    return {
      TestRecipient,
      TestTokenRecipient,
    };
  }
}
