pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // nums.len() > nums.iter().collect::<HashSet<_>>().len()

    use std::collections::HashSet;
    let mut seen_once = HashSet::new();

    for num in nums {
        if seen_once.contains(&num) {
            return true;
        } else {
            seen_once.insert(num);
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }
}
