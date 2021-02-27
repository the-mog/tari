#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_crypto::common::Blake256;
use tari_mmr::merkle_mountain_range::MerkleMountainRange;

pub type Hasher = Blake256;

fuzz_target!(|data: &[u8]| {
    let mut mmr = MerkleMountainRange::<Hasher, _>::new(Vec::default());

    if let Ok(s) = std::str::from_utf8(data){

        if let Ok(u) = s.parse::<usize>(){
            let h = Hasher::new();
            let hh = h.result().to_vec();
            //
            //let hash = Hasher::digest(&u.to_le_bytes()).to_vec();
            //println!("{:?}", &hh);
            let push_hash = mmr.push(hh);
            assert!(push_hash.is_ok());
            let _get_lhash = mmr.get_leaf_hash(u);
            let _ge_lhashes = mmr.get_leaf_hashes(u,u);
            let _get_nh = mmr.get_node_hash(u);


        }
    }
//not sure if hashes are checked for validity
    let _find_lidx = mmr.find_leaf_index(data);
    let _find_nidx = mmr.find_node_index(data);
    //run this one on its own fuzz file
    //create new mmr, push, assign
    //let assing = mmr.assign(state)



});
