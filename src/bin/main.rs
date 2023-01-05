// use std::env;
// use std::str::FromStr;
use std::time::Instant;
// use num_bigint::BigUint;

fn main() {
    let start = Instant::now();
    /* 
    let mut num = BigUint::from(10000000u32);
    for arg in env::args().skip(1) {
        num = BigUint::from_str(&arg).unwrap();
        break;

    }
    println!("{}", num);
    let mut i = BigUint::from(0u32);
    while i < num {
       i += BigUint::from(1u32); 
    }
    */
    let num = 1000000000u64;
    println!("{}", num);
    let mut i = 0;
    while i < num {
        i += 1;
    }
    let end = start.elapsed();
    println!("time: {}.{:03}", end.as_secs(), end.subsec_nanos()/1000000);
    
}
