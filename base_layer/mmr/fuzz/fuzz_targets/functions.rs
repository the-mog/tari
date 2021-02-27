#![no_main]
use libfuzzer_sys::fuzz_target;

use croaring::Bitmap;
use tari_crypto::common::Blake256;
use tari_mmr::{mutable_mmr::MutableMmr,merkle_mountain_range::MerkleMountainRange};
use tari_mmr::{functions, Hash};

pub type Hasher = Blake256;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mut mut_mmr = MutableMmr::<Hasher, _>::new(Vec::default(), Bitmap::create()).unwrap();
    let mmr = MerkleMountainRange::<Hasher, _>::new(Vec::default());
    
    if let Ok(s) = std::str::from_utf8(data){
        if let Ok(u) = s.parse::<u8>(){
            let mut addhash: Hash = Vec::new();
            addhash.push(u);
            let mut additions = Vec::new();
            additions.push(addhash);

            let h = Hasher::new();
            let hh = h.result().to_vec();
            let _push_hash = mut_mmr.push(hh);
            
            if let Ok(x) = s.parse::<u32>(){
                let mut deletions = Vec::new();
                deletions.push(x);
               
                let _calc_p_mmr_root = functions::calculate_pruned_mmr_root(&mut_mmr, additions.clone(), deletions);
                let _calc_mmr_root   = functions::calculate_mmr_root(&mmr, additions);
                let _p_mmr = functions::prune_mmr(&mmr);
                let _p_mut_mmr = functions::prune_mutable_mmr(&mut_mmr);

            }




        }
    }
    //let backend = PrunedHashSet::try_from(src_mmr);
   // let mut p_mmr = functions::prune_mmr(&mmr);
   // let mut p_mut_mmr = functions::prune_mutable_mmr(&mut_mmr);
});
