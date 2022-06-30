pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut p = nums.len() - 1;
    let mut res = vec![0; nums.len()];

    while start <= end {
        if nums[start] * nums[start] < nums[end] * nums[end] {
            res[p] = nums[end] * nums[end];
            end -= 1;
        } else {
            res[p] = nums[start] * nums[start];
            start += 1;
        }

        if p == 0 {
            break;
        }
        p -= 1
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
}
