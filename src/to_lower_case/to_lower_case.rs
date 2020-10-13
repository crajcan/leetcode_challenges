pub fn to_lower_case(str: String) -> String {
    str.bytes()
        .map(|c| char::from(if c > 64 && c < 91 { c + 32 } else { c }))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_lower_case() {
        assert_eq!(to_lower_case("Hello".to_string()), "hello");
        assert_eq!(to_lower_case("here".to_string()), "here");
        assert_eq!(to_lower_case("LOVELY".to_string()), "lovely");
        assert_eq!(to_lower_case("al&phaBET".to_string()), "al&phabet");
    }
}
