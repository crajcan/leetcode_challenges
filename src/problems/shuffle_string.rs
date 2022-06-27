pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut res_vec = vec!['a'; s.len()];

    for (i, c) in s.chars().enumerate() {
        res_vec[indices[i] as usize] = c
    }

    res_vec.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_restore_string() {
        assert_eq!(
            restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
        assert_eq!(
            restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]),
            "nihao".to_string()
        );
    }
}
