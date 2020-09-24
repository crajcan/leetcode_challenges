pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    vec![42]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
       assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
       assert_eq!(running_sum(vec![1, 2, 3, 4]),    [1, 3, 6, 10]);
    }
}
