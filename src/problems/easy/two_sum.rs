pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut seen = HashMap::new();

    for i in 0..nums.len() {
        let needed = target - nums[i];

        if seen.contains_key(&needed) {
            return vec![seen[&needed], i as i32];
        } else {
            seen.insert(nums[i], i as i32);
        }
    }

    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![14, 2, 4, 5, 6, 7], 8), vec![1, 4]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
