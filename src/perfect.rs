pub fn is_perfect(i: usize) -> bool {
    if i < 2 {
        return false;
    }

    let mut sum: usize = 1;

    let sq = (i as f64).sqrt() as usize + 1;
    for a in 2..sq {
        if i % a == 0 {
            sum += a;
            let ia = i / a;
            if ia != a {
                sum += ia;
            }
        }
    }

    return sum == i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_perfect_test() {
        let true_cases: [usize; 5] = [
            6, 28, 496, 8128, 33550336,
        ];
        for i in true_cases.iter() {
            assert_eq!(is_perfect(*i), true, "{} is perfect", *i);
        }

        let false_cases: [usize; 5] = [
            1, 2, 3, 4, 5,
        ];
        for i in false_cases.iter() {
            assert_eq!(is_perfect(*i), false, "{} is not perfect", *i);
        }
    }
}
