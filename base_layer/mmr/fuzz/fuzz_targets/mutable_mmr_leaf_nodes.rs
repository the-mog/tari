#![no_main]
use libfuzzer_sys::fuzz_target;

use croaring::Bitmap;
use tari_mmr::mutable_mmr_leaf_nodes::MutableMmrLeafNodes;


fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mut _mmr = MutableMmrLeafNodes::new(Vec::default(), Bitmap::create());


});
