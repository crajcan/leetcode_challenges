pub fn single_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut seen_once = HashMap::new();

    for num in nums {
        if let std::collections::hash_map::Entry::Vacant(e) = seen_once.entry(num) {
            e.insert(1);
        } else {
            seen_once.remove(&num);
        }
    }

    *seen_once.keys().next().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
