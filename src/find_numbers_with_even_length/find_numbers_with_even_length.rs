pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |sum, num| {
        if num.to_string().len() % 2 == 0 {
            sum + 1
        } else {
            sum
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_numbers() {
        assert_eq!(find_numbers(vec![1]), 0);
        assert_eq!(find_numbers(vec![20]), 1);
        assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    }
}
