import type { Address } from '@ortege/utils';

export enum HookContractType {
  HOOK = 'hook',
  ISM = 'ism',
}

export type MessageHookConfig = {
  hookContractType: HookContractType.HOOK;
  nativeBridge: Address;
  remoteIsm: Address;
  destinationDomain: number;
};

export type NoMetadataIsmConfig = {
  hookContractType: HookContractType.ISM;
  nativeBridge: Address;
};

export type HookConfig = MessageHookConfig | NoMetadataIsmConfig;
