#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_mmr::merkle_proof::MerkleProof;
use tari_mmr::MerkleMountainRange;
use tari_crypto::common::Blake256;

pub type Hasher = Blake256;


fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mut mmr = MerkleMountainRange::<Hasher, _>::new(Vec::default());
    if let Ok(s) = std::str::from_utf8(data){

        if let Ok(u) = s.parse::<usize>(){
            let h = Hasher::new();
            let hash = h.result().to_vec();
            //
            //let hash = Hasher::digest(&u.to_le_bytes()).to_vec();
            //println!("{:?}", &hh);
            let _push_hash = mmr.push(hash);
            let root = mmr.get_merkle_root().unwrap();
            let lpos = u;
            let proof = MerkleProof::for_leaf_node(&mmr, lpos).unwrap();
            let proof_node = MerkleProof::for_node(&mmr, lpos).unwrap();
            let _ = proof.verify_leaf::<Hasher>(&root, &data, lpos);
            let _ = proof_node.verify::<Hasher>(&root, &data, lpos);


        }
    }
//not sure if hashes are checked for validity
    let _find_lidx = mmr.find_leaf_index(data);
    let _find_nidx = mmr.find_node_index(data);
    //run this one on its own fuzz file
    //create new mmr, push, assign
    //let assing = mmr.assign(state)


});
