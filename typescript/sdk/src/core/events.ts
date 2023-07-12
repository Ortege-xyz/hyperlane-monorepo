import type {
  DispatchEvent,
  ProcessEvent,
} from '@ortege/core/dist/contracts/Mailbox';

export { DispatchEvent, ProcessEvent };

export type HyperlaneLifecyleEvent = ProcessEvent | DispatchEvent;
