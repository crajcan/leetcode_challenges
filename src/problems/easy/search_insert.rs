pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;
    let mut lo = 0;
    let mut hi = nums.len();

    while lo < hi {
        let mid = lo + ((hi - lo) / 2);

        match target.cmp(&nums[mid]) {
            Ordering::Greater => lo = mid + 1,
            Ordering::Less => hi = mid,
            Ordering::Equal => return mid as i32,
        }
    }
    println!("lo: {}, hi: {}", lo, hi);
    if lo == nums.len() {
        return nums.len() as i32;
    }

    if target < nums[lo] as i32 {
        lo as i32
    } else {
        (lo + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
