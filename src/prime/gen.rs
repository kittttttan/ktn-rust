/// generate prime numbers
pub fn sieve_max(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    if n < 2 {
        return vec;
    }

    let upper = n + 1;
    let upper_sqrt = ((n as f64).sqrt().floor() as u32) + 1;
    dbg!(upper, upper_sqrt);

    let mut s = vec![false; upper as usize];
    for z in (1..6).step_by(4) {
        for y in (z..upper_sqrt).step_by(6) {
            for x in 1..upper_sqrt {
                let n = 4 * x * x + y * y;
                if n >= upper {
                    break;
                }
                s[n as usize] = !s[n as usize];
            }
            for x in (y+1..upper_sqrt).step_by(2) {
                let n = 3 * x * x - y * y;
                if n >= upper {
                    break;
                }
                s[n as usize] = !s[n as usize];
            }
        }
    }
    for z in (2..5).step_by(2) {
        for y in (z..upper_sqrt).step_by(6) {
            for x in (1..upper_sqrt).step_by(2) {
                let n = 3 * x * x + y * y;
                if n >= upper {
                    break;
                }
                s[n as usize] = !s[n as usize];
            }
            for x in (y+1..upper_sqrt).step_by(2) {
                let n = 3 * x * x - y * y;
                if n >= upper {
                    break;
                }
                s[n as usize] = !s[n as usize];
            }
        }
    }
    for y in (3..upper_sqrt).step_by(6) {
        for z in 1..3 {
            for x in (z..upper_sqrt).step_by(3) {
                let n = 4 * x * x + y * y;
                if n >= upper {
                    break;
                }
                s[n as usize] = !s[n as usize];
            }
        }
    }

    for i in 5..upper_sqrt {
        if s[i as usize] {
            let x = i * i;
            for j in (x..upper).step_by(x as usize) {
                s[j as usize] = false;
            }
        }
    }
    s[2] = true;
    s[3] = true;

    for i in 0..upper {
        if s[i as usize] {
            vec.push(i);
        }
    }

    return vec;
}

pub fn factorial(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();

    if n < 2 {
        return vec;
    }

    let upper_sqrt = ((n as f64).sqrt().floor() as u32) + 1;
    let primes = sieve_max(upper_sqrt);
    let mut a = n;
    for p in primes {
        while a % p == 0 {
            vec.push(p);
            a /= p;
        }
        if a < p {
            break;
        }
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve_max_test() {
        let a: [u32; 25] = [
            2, 3, 5, 7,
            11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47,
            53, 59, 61, 67,
            71, 73, 79, 83, 89, 97,
        ];
        let s = sieve_max(100);
        assert_eq!(s.len(), 25);

        for i in 0..25 {
            assert_eq!(s[i], a[i]);
        }
    }

    #[test]
    fn factorial_test() {
        let case = 100u32;
        let a: [u32; 4] = [2, 2, 5, 5];

        let fs = factorial(case);
        assert_eq!(fs.len(), 4);
        for i in 0..4 {
            assert_eq!(fs[i], a[i]);
        }
    }
}
