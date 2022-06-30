pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }
}
