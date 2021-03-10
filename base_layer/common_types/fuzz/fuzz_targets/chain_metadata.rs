#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_common_types::types::BlockHash;
use tari_common_types::chain_metadata::ChainMetadata;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(u) = s.parse::<u64>(){
            if let Ok(x) = s.parse::<u128>(){
                let cm = ChainMetadata::new(u,BlockHash,u,u,x);
                let _hb = cm.horizon_block(u);
                let _ = cm.set_pruning_horizon(u);

            }
        }
    }
});
