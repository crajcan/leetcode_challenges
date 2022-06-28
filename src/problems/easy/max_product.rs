use std::cmp;

pub fn max_product(nums: Vec<i32>) -> i32 {
    let maximums = nums.iter().fold((0, 0), |maximums, x| {
        match (maximums.0 < *x, maximums.1 < *x) {
            (false, false) => maximums,
            (true, false) => (*x, maximums.1),
            _ => (cmp::max(maximums.0, maximums.1), *x),
        }
    });

    (maximums.0 - 1) * (maximums.1 - 1)
}

mod test {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(max_product(vec![3, 7]), 12);
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
    }
}
