pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    match candies.iter().max() {
        Some(max) => candies
            .clone()
            .into_iter()
            .map(|n| {
                if n + extra_candies < *max {
                    false
                } else {
                    true
                }
            })
            .collect(),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_with_candies() {
        assert_eq!(kids_with_candies(vec![], 3), vec![]);
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            [true, true, true, false, true]
        );
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            [true, false, false, false, false]
        );
    }
}
