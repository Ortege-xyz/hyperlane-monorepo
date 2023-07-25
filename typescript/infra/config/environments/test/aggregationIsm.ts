import { AggregationIsmConfig, ModuleType } from '@ortege/sdk';

import { merkleRootMultisig, messageIdMultisig } from './multisigIsm';

export const aggregationIsm = (validatorKey: string): AggregationIsmConfig => {
  return {
    type: ModuleType.AGGREGATION,
    modules: [
      merkleRootMultisig(validatorKey),
      messageIdMultisig(validatorKey),
    ],
    threshold: 1,
  };
};
