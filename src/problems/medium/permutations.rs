pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    match nums.len() {
        0 => panic!("shouldn't have empty input"),
        1 => vec![nums],
        _ => {
            let mut permutations = vec![];

            for num in &nums[..] {
                let mut nums_without_num = nums.clone();
                nums_without_num.retain(|n| n != num);

                let permutations_without_num = permute(nums_without_num);

                permutations_without_num.iter().for_each(|permutation| {
                    permutations.push([vec![*num], permutation.to_vec()].concat());
                })
            }

            permutations
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
