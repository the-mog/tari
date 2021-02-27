#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_mmr::common;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = std::str::from_utf8(data){
        if let Ok(n) = s.parse::<usize>(){
            let _n_index = common::node_index(n);
            let _l_index = common::leaf_index(n);
            let _is_leaf = common::is_leaf(n);
            let _find_peaks= common::find_peaks(n);
            let _family = common::family(n);
            let _famaily_branch = common::family_branch(n,n);
            let _bintree_height = common::bintree_height(n);
            let _peak_map_height = common::peak_map_height(n);
            let _peak_sizes_height = common::peak_sizes_height(n);
            let _is_left_sibling = common::is_left_sibling(n);
            let _n_leaves = common::n_leaves(n);

        }
        //let _ht t = common::hash_together(data,data);

    }
});
