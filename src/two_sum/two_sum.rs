fn compare_with_rest(i: usize, num: i32, nums: &[i32], target: i32) -> Option<Vec<usize>> {
    for (j, other_num) in nums[i+1..].iter().enumerate() {
        if num + other_num == target {
            return Some(vec![i, (j + i+1)]);
        }
    }

    None
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num) in nums.iter().enumerate() {
        match compare_with_rest(i, *num, nums.as_slice(), target){
            Some(result) => return result.into_iter().map(|x| x as i32).collect(),
            None         => ()
        }
    }

    vec![]
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2,7,11,15],    9), vec![0,1]);
        assert_eq!(two_sum(vec![14,2,4,5,6,7], 8), vec![1,4]);
        assert_eq!(two_sum(vec![3,2,4],        6), vec![1,2]);
    }
}
