pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut total: i32 = nums.iter().sum();

    for i in 0..nums.len() {
        total -= nums[i];

        if sum == total {
            return i as i32;
        }

        sum += nums[i];
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pivot_index() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(pivot_index(vec![-1, -1, -1, 0, 1, 1]), 0);
    }
}
