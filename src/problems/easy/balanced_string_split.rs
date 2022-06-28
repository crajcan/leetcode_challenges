pub fn balanced_string_split(s: String) -> i32 {
    let mut balance = 0;
    let mut count = 0;

    for char in s.chars() {
        if char == 'L' {
            balance += 1;
        } else {
            balance -= 1;
        }

        if balance == 0 {
            count += 1
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balanced_string_split() {
        assert_eq!(balanced_string_split("RLRRLLRLRL".to_string()), 4);
        assert_eq!(balanced_string_split("RLLLLRRRLR".to_string()), 3);
        assert_eq!(balanced_string_split("LLLLRRRR".to_string()), 1);
        assert_eq!(balanced_string_split("RLRRRLLRLL".to_string()), 2);
    }
}
