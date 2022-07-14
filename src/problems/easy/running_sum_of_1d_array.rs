pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = (0..nums.len() as i32).collect();
    let mut sum = 0;

    for i in 0..res.len() {
        sum += nums[i];
        res[i] = sum;
    }

    res
}

pub fn functional_running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().fold(vec![], |sums, num| {
        [
            &sums[..],
            &[num
                + match sums.last() {
                    Some(prior_sum) => *prior_sum,
                    None => 0,
                }],
        ]
        .concat()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
        assert_eq!(running_sum(vec![1, 2, 3, 4]), [1, 3, 6, 10]);
    }

    #[test]
    fn test_functional_running_sum() {
        assert_eq!(functional_running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
        assert_eq!(functional_running_sum(vec![1, 2, 3, 4]), [1, 3, 6, 10]);
    }
}
