pub fn helper(s: &[u8], t: &[u8]) -> bool {
    match (s.len(), t.len()) {
        (0, _) => true,
        (_, 0) => false,
        (_, _) => match s[0] == t[0] {
            false => helper(s, &t[1..]),
            true => helper(&s[1..], &t[1..]),
        },
    }
}

pub fn is_subsequence(s: String, t: String) -> bool {
    helper(&s.as_bytes(), &t.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert_eq!(is_subsequence("abc".to_string(), "absc".to_string()), true);
        assert_eq!(
            is_subsequence("abc".to_string(), "acbsd".to_string()),
            false
        );
        assert_eq!(is_subsequence("".to_string(), "absc".to_string()), true);
        assert_eq!(is_subsequence("".to_string(), "".to_string()), true);
        assert_eq!(is_subsequence("a".to_string(), "".to_string()), false);
    }
}
