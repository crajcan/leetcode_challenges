pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = (0..nums.len() as i32).collect();
    let mut sum = 0;

    for i in 0..res.len() {
      sum = sum + nums[i];
      res[i] = sum;
    }

    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
       assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
       assert_eq!(running_sum(vec![1, 2, 3, 4]),    [1, 3, 6, 10]);
    }
}
