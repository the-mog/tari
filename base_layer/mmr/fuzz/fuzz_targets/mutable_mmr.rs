#![no_main]
use libfuzzer_sys::fuzz_target;

use croaring::Bitmap;
use tari_crypto::common::Blake256;
use tari_mmr::mutable_mmr::MutableMmr;

pub type Hasher = Blake256;

fuzz_target!(|data: &[u8]| {
    let mut mmr = MutableMmr::<Hasher, _>::new(Vec::default(), Bitmap::create()).unwrap();

    if let Ok(s) = std::str::from_utf8(data){

        if let Ok(u) = s.parse::<usize>(){
            let h = Hasher::new();
            let hh = h.result().to_vec();
            //
            //let hash = Hasher::digest(&u.to_le_bytes()).to_vec();
            //println!("{:?}", &hh);
            let push_hash = mmr.push(hh);
            assert!(push_hash.is_ok());
            let get_lhash = mmr.get_leaf_hash(u as u32);
            println!("{:?}", &get_lhash);
            //assert_eq!(get_lhash, hh);
            let get_lstatus = mmr.get_leaf_status(u as u32);
            println!("{:?}", &get_lstatus);
            let _del = mmr.delete(u as u32);
            let _to_lf = mmr.to_leaf_nodes(u,u);


        }
    }
//not sure if hashes are checked for validity
    let _find_lidx = mmr.find_leaf_index(data);
    let _find_nidx = mmr.find_node_index(data);
    //run this one on its own fuzz file
    //create new mmr, push, assign
    //let assing = mmr.assign(state)


});

