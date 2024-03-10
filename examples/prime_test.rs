use ktn_rust::prime;
use std::{io, num::ParseIntError};

fn main() -> Result<(), ParseIntError> {
    let mut s = String::new();
    loop {
        io::stdin().read_line(&mut s).expect("input error");
        if s.trim_end().is_empty() {
            break;
        }

        // dbg!(s.clone());
        let n = s.trim_end().parse::<u32>().expect("input number");
        let b = prime::test::try_division(n);
        println!("{n} is prime: {b}");
        s.clear();
    }

    Ok(())
}
