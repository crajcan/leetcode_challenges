pub fn rob_helper(nums: &[i32], seen_totals: &mut Vec<i32>, i: usize) -> i32 {
    match nums.len() {
        0 => 0,
        1 => nums[0],
        2 => std::cmp::max(nums[0], nums[1]),
        _ => {
            let first_plus_total_after_two = match seen_totals[i] {
                -1 => {
                    let res = nums[0] + rob_helper(&nums[2..], seen_totals, i + 2);

                    seen_totals[i] = res;

                    res
                }
                res => res,
            };

            let second_plus_total_after_three = match seen_totals[i + 1] {
                -1 => {
                    let res = nums[1] + rob_helper(&nums[3..], seen_totals, i + 3);

                    seen_totals[i + 1] = res;

                    res
                }
                res => {
                    res
                }
            };

            std::cmp::max(first_plus_total_after_two, second_plus_total_after_three)
        }
    }
}

pub fn rob(nums: Vec<i32>) -> i32 {
    rob_helper(&nums, &mut vec![-1; nums.len()], 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![5, 2, 7, 9, 3, 5]), 19);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(vec![1, 3, 1]), 3);
    }
}
