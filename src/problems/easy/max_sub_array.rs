pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut global_max = i32::MIN;
    let mut local_max = 0;
    for num in nums {
        local_max = std::cmp::max(num, local_max + num);
        global_max = std::cmp::max(global_max, local_max);
    }
    global_max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}