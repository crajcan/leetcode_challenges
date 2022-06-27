pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = (0..nums.len() as i32).collect();
    let mut sum = 0;

    for i in 0..res.len() {
        sum = sum + nums[i];
        res[i] = sum;
    }

    res
}

fn running_sum_helper(nums: Vec<i32>, sum: i32) -> Vec<i32> {
    match nums.as_slice() {
        [] => vec![],
        _ => [
            vec![nums[0] + sum],
            running_sum_helper(nums[1..].to_vec(), nums[0] + sum),
        ]
        .concat(),
    }
}

pub fn functional_running_sum(nums: Vec<i32>) -> Vec<i32> {
    running_sum_helper(nums, 0)
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
