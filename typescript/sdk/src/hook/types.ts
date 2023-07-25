import type { types } from '@ortege/utils';

export enum HookContractType {
  HOOK = 'hook',
  ISM = 'ism',
}

export type MessageHookConfig = {
  hookContractType: HookContractType.HOOK;
  nativeBridge: types.Address;
  remoteIsm: types.Address;
  destinationDomain: number;
};

export type NoMetadataIsmConfig = {
  hookContractType: HookContractType.ISM;
  nativeBridge: types.Address;
};

export type HookConfig = MessageHookConfig | NoMetadataIsmConfig;
