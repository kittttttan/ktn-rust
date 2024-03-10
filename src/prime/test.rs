use rand::{thread_rng, Rng};

pub fn try_division(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let sq = ((n as f64).sqrt().floor() as u32) + 1;
    for i in (3..sq).step_by(2) {
        let m = n % i;
        // dbg!(n, i, m);
        if m == 0 {
            return false;
        }
    }

    return true;
}

pub fn mod_pow(base: u32, power: u32, m: u32) -> u32 {
    let mut result = 1;
    let mut b = base;
    let mut p = power;

    while p > 0 {
        if p & 1 > 0 {
            result = (result * b) % m;
        }
        b = (b * b) % m;
        p >>= 1;
    }

    return result;
}

/// Miller–Rabin primality test
pub fn mrpt(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }

    let mut d = n - 1;
    while d & 1 == 0 {
        d >>= 1;
    }

    let mut rng = thread_rng();
    let mut i = 20;
    while i > 0 {
        i -= 1;
        let a = rng.gen_range(1..n-1);
        let mut t = d;
        let mut y = mod_pow(a, t, n);
        dbg!(a, t, n, y);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y * y) % n;
            t <<= 1;
        }
        dbg!(y, t);
        if y != n - 1 && t & 1 == 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_division_test() {
        let true_cases: [u32; 5] = [
            2, 3, 5, 97, 997,
        ];
        for i in true_cases.iter() {
            assert_eq!(try_division(*i), true, "{} is prime", *i);
        }

        let false_cases: [u32; 4] = [
            1, 4,
            91, // 7*13
            987, // 3*329
        ];
        for i in false_cases.iter() {
            assert_eq!(try_division(*i), false, "{} is not prime", *i);
        }
    }
}
