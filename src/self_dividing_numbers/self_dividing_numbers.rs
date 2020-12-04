fn digits(num: i32) -> Vec<i32> {
    match num {
        0 => vec![],
        _ => [digits(num / 10), vec![num % 10]].concat()
    }
}

fn is_self_dividing(num: i32) -> bool {
    digits(num).iter().all(|&digit| digit != 0 && num % digit == 0)
}

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right).filter(|&num| is_self_dividing(num)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(0), vec![]);
        assert_eq!(digits(1), vec![1]);
        assert_eq!(digits(128), vec![1, 2, 8]);
    }

    #[test]
    fn test_is_self_dividing() {
        assert_eq!(is_self_dividing(1),  true);
        assert_eq!(is_self_dividing(2),  true);
        assert_eq!(is_self_dividing(9),  true);
        assert_eq!(is_self_dividing(10), false);
        assert_eq!(is_self_dividing(11), true);
        assert_eq!(is_self_dividing(12), true);
        assert_eq!(is_self_dividing(14), false);
    }

    #[test]
    fn test_self_dividing_numbers() {
        assert_eq!(self_dividing_numbers(1, 22), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]);
    }
}
