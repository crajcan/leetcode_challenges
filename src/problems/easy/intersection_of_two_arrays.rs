pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut uniq = HashMap::new();

    for num in nums2 {
        *uniq.entry(num).or_insert(0) += 1;
    }
    println!("uniq: {:?}", uniq);

    let mut res = vec![];
    for num in nums1 {
        if uniq.contains_key(&num) && uniq[&num] > 0 {
            res.push(num);
            uniq.entry(num).and_modify(|v| *v -= 1);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    }
}
