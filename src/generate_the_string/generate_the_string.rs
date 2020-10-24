pub fn generate_the_string(n: i32) -> String {
    if n == 1 {
        return "a".to_string();
    }

    match n % 2 {
        0 => [
            "a",
            &String::from_utf8(vec![b'b'; (n - 1) as usize])
                .unwrap()
                .to_owned(),
        ]
        .join(""),
        _ => [
            "a",
            "b",
            &String::from_utf8(vec![b'c'; (n - 2) as usize])
                .unwrap()
                .to_owned(),
        ]
        .join(""),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_the_string() {
        assert_eq!(generate_the_string(2), "ab".to_string());
        assert_eq!(generate_the_string(4), "abbb".to_string());
        assert_eq!(generate_the_string(7), "abccccc".to_string());
    }
}
