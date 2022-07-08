pub fn is_palindrome(s: String) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
    let chars = s.chars().collect::<Vec<char>>();

    while left < right {
        match (
            chars[left].is_ascii_alphanumeric(),
            chars[right].is_ascii_alphanumeric(),
        ) {
            (false, false) => {
                left += 1;
                right -= 1
            }
            (false, true) => left += 1,
            (true, false) => right -= 1,
            (true, true) => {
                if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                    return false;
                }

                left += 1;
                right -= 1;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("Able was I ere I saw Elba".to_string()), true);
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(
            is_palindrome("A man, a plan, ia canal: Panama".to_string()),
            false
        );
    }
}
