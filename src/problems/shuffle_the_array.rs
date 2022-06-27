pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut res = vec![];

    for i in 0..n {
        res.push(nums[i as usize]);
        res.push(nums[(i + n) as usize]);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        assert_eq!(shuffle(vec![1, 1, 2, 2], 2), [1, 2, 1, 2]);
        assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), [2, 3, 5, 4, 1, 7]);
        assert_eq!(
            shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1,], 4),
            [1, 4, 2, 3, 3, 2, 4, 1]
        );
    }
}
