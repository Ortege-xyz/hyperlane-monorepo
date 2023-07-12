import { RouterConfig, chainMetadata } from '@ortege/sdk';

export type HelloWorldConfig = RouterConfig;

// SET DESIRED NETWORKS HERE
export const prodConfigs = {
  alfajores: chainMetadata.alfajores,
};
