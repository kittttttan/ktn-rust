use ktn_rust::collatz;
use std::{env, num::ParseIntError};

fn main() {
    let ns = env::args().nth(1).expect("no input number");
    let n: usize = ns.parse().expect("input number > 0");
    println!("collatz {n}");
    let cnt = collatz::collatz(n);
    println!("{cnt}");
}
