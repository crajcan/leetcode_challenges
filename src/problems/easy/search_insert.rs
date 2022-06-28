pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let len: i32 = nums.len() as i32;
    
    let res: i32 = nums.into_iter().enumerate().fold(-1, |acc, (i, j)| {
            if target <= j && acc == -1 {
                i as i32 
            } else {
                acc
            }
        }
    );
    
    if res == -1 {
        return len;
    } 
    
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
    }
}