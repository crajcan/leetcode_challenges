pub fn is_power_of_two(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    while n > 1 {
        if n % 2 == 1 {
            return false;
        }
        n /= 2;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert_eq!(is_power_of_two(1), true);
        assert_eq!(is_power_of_two(2), true);
        assert_eq!(is_power_of_two(3), false);
        assert_eq!(is_power_of_two(4), true);
        assert_eq!(is_power_of_two(5), false);
        assert_eq!(is_power_of_two(6), false);
        assert_eq!(is_power_of_two(7), false);
        assert_eq!(is_power_of_two(8), true);
        assert_eq!(is_power_of_two(9), false);
        assert_eq!(is_power_of_two(10), false);
        assert_eq!(is_power_of_two(11), false);
        assert_eq!(is_power_of_two(12), false);
        assert_eq!(is_power_of_two(13), false);
        assert_eq!(is_power_of_two(14), false);
        assert_eq!(is_power_of_two(15), false);
        assert_eq!(is_power_of_two(16), true);
        assert_eq!(is_power_of_two(17), false);
    }
}
