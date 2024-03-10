use ktn_rust::prime;
use std::{env, num::ParseIntError};

fn main() {
    let ns = env::args().nth(1).expect("no input number");
    let max: u32 = ns.parse().expect("input number > 0");
    println!("primes below {max}");
    let ps = prime::gen::sieve_max(max);
    for p in ps {
        println!("{p}");
    }
}
