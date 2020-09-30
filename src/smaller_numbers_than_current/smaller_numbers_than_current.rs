fn smaller_than_index(nums: &Vec<i32>, index: usize) -> i32 {
    let mut sum = 0;
    let elem = nums[index];

    for i in 0..nums.len() {
        if i != index && nums[i] < elem {
            sum += 1;
        }
    }

    sum
}

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .map(|(i, _x)| smaller_than_index(&nums, i))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
}
