pub fn sum_zero(n: i32) -> Vec<i32> {
    let mut start = 0;
    let mut result = vec![];

    if n % 2 == 1 {
        result.push(0);
        start = 1;
    }

    let mut e = 1;
    for _i in start.clone()..n {
        result.push(e);
        e *= -1;

        if e > 0 {
            e += 1;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_zero() {
        assert_eq!(sum_zero(1), vec![0]);
        assert_eq!(sum_zero(3), vec![0, 1, -1]);
        assert_eq!(sum_zero(4), vec![1, -1, 2, -2]);
        assert_eq!(sum_zero(5), vec![0, 1, -1, 2, -2]);
    }
}
