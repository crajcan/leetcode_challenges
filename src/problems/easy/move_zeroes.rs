pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut where_to_write: usize = 0;

    for ptr in 0..nums.len() {
        if nums[ptr] != 0 {
            let temp = nums[where_to_write];
            nums[where_to_write] = nums[ptr];
            nums[ptr] = temp;
            where_to_write += 1;
        }
    }
}

pub fn move_zeroes_recursive(nums: &mut Vec<i32>) {
    move_zeroes_recursive_helper(nums, 0, 0);
}

pub fn swap(nums: &mut Vec<i32>, ptr: usize, where_to_write: usize) {
    let temp = nums[where_to_write];
    nums[where_to_write] = nums[ptr];
    nums[ptr] = temp;
}

pub fn move_zeroes_recursive_helper(nums: &mut Vec<i32>, ptr: usize, where_to_write: usize) {
    if ptr == nums.len() {
        ()
    } else {
        if nums[ptr] != 0 {
            swap(nums, ptr, where_to_write);
            move_zeroes_recursive_helper(nums, ptr + 1, where_to_write + 1);
        } else {
            move_zeroes_recursive_helper(nums, ptr + 1, where_to_write);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_recursive() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes_recursive(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
