pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut where_to_write = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[where_to_write] = nums[i];
            where_to_write += 1;
        }
    }

    where_to_write as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(2, remove_element(&mut nums, 3));
        assert_eq!(vec![2, 2], nums[..2]);

        nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(5, remove_element(&mut nums, 2));
        assert_eq!(vec![0, 1, 3, 0, 4], nums[..5]);
    }
}
