pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    use std::collections::HashMap;
    
    let mut indices = HashMap::new();
    for i in 0..nums.len() {
        if let Some(j) = indices.get(&nums[i]) {
            if i - j <= k {
                return true;
            } 
        }
    
        indices.insert(nums[i], i);
    }

    false 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(
            contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
        assert_eq!(
            contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}