import {
  DomainRoutingIsmFactory__factory,
  StaticAggregationIsmFactory__factory,
  StaticMerkleRootMultisigIsmFactory__factory,
  StaticMessageIdMultisigIsmFactory__factory,
} from '@ortege/core';

export const ismFactoryFactories = {
  merkleRootMultisigIsmFactory:
    new StaticMerkleRootMultisigIsmFactory__factory(),
  messageIdMultisigIsmFactory: new StaticMessageIdMultisigIsmFactory__factory(),
  aggregationIsmFactory: new StaticAggregationIsmFactory__factory(),
  routingIsmFactory: new DomainRoutingIsmFactory__factory(),
};

export type IsmFactoryFactories = typeof ismFactoryFactories;
