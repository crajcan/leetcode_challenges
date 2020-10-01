pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    match nums.as_slice() {
        [] => vec![],
        _ => [
            vec![nums[1]; nums[0] as usize],
            decompress_rl_elist(nums[2..].to_vec()),
        ]
        .concat(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decompress_rl_elist() {
        assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
        assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
    }
}
