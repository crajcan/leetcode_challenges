pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    s.chars().fold(
        0,
        |acc, stone| if j.contains(stone) { acc + 1 } else { acc },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones() {
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
    }
}
