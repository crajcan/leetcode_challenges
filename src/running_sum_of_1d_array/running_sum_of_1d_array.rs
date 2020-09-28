pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = (0..nums.len() as i32).collect();
    let mut sum = 0;

    for i in 0..res.len() {
        sum = sum + nums[i];
        res[i] = sum;
    }

    res
}
/*
                 println!("nums[0]: {:?}",   nums[0]);
                 println!("nums[1..]: {:?}", &nums[1..]);
                 let () = &nums[1..];
                 vec![]
fn running_sum_helper(nums: &[i32], sum: i32) -> &'static [i32] {
    match nums {
        [] => &[],
        _ => &[nums[0]]
    }
}

pub fn functional_running_sum(nums: Vec<i32>) -> Vec<i32> {
    running_sum_helper(nums.as_slice(), 0).to_vec()
}

*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
        assert_eq!(running_sum(vec![1, 2, 3, 4]), [1, 3, 6, 10]);
    }
    /*
        #[test]
        fn test_functional_running_sum() {
            assert_eq!(functional_running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
             assert_eq!(functional_running_sum(vec![1, 2, 3, 4]),    [1, 3, 6, 10]);
        }
    */
}
