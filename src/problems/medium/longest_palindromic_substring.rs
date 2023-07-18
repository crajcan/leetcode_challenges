pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();

    for len in (1..=s.len()).rev() {
        for substring in bytes.windows(len) {
            if is_palindrome(substring) {
                return String::from_utf8(substring.to_vec()).unwrap()
            }
        }
    }

    "".to_string() 
}

fn is_palindrome(s: &[u8]) -> bool {
    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - 1 - i] {
            return false
        }
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn test_longest_palindrome() {
        assert_eq!(super::longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(super::longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(super::longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(super::longest_palindrome("ac".to_string()), "a".to_string());
    }
}