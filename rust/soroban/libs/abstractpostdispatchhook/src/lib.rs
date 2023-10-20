use soroban_sdk::{Bytes};
use standardhookmetadata::{StandardHookMetadata, VARIANT};

pub struct  AbstractPostDispatchHook;

impl AbstractPostDispatchHook {
    pub fn supports_metadata(metadata: Bytes) -> bool {
        return metadata.len() == 0 || StandardHookMetadata::variant(metadata) == VARIANT;
    }
}
