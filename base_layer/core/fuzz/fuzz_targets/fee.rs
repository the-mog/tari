#![no_main]
use libfuzzer_sys::fuzz_target;

use tari_core::transactions::fee::Fee;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = std::str::from_utf8(data){
        if let Ok(x) = s.parse::<usize>()
            Fee::calculate_weight(x,x,x,x);
    }
});
