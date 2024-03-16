pub fn collatz(mut i: usize) -> usize {
    if i < 1 {
        return 0;
    }

    let mut cnt = 1;

    while i != 1 {
        dbg!(i);
        cnt += 1;
        if i % 2 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }
    }
    dbg!(i);

    return cnt;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_test() {
        let true_cases: [usize; 4] = [
            0, 1, 2, 8,
        ];
        for (i, c) in true_cases.iter().enumerate() {
            assert_eq!(collatz(i), *c);
        }
    }
}
