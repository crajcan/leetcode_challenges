pub fn single_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut seen_once = HashMap::new();

    for num in nums {
        if seen_once.contains_key(&num) {
            seen_once.remove(&num);
        } else {
            seen_once.insert(num, 1);
        }
    }

    *seen_once.keys().nth(0).unwrap()
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
