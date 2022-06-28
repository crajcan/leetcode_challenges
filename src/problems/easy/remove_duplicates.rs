pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut previous = None;
    let mut j = 0;

    for i in 0..nums.len() {
        if (Some(nums[i])) != previous {
            previous = Some(nums[i]);
            nums.swap(i, j);
            j += 1;
        }
    }

    j as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates(&mut nums));
        assert_eq!(vec![1, 2, 1], nums);

        nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates(&mut nums));
        assert_eq!(vec![0, 1, 2, 3, 4, 0, 2, 1, 3, 1], nums);
    }
}
