pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    let mut result = "".to_string();
    if strs.len() == 0 { return result };
    let mut i = 0;
    
    while i < strs[0].len() {
        let this_char = strs[0].as_bytes().get(i).unwrap();
        
        for str in &strs[1..] {
            let this_char_in_str = match str.as_bytes().get(i) {
                Some(c) => c,
                None => return result
            };
            
            if this_char != this_char_in_str  {
                return result
            }
        }
        
        result.push(*this_char as char);
        i = i + 1;
    }
    
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec!["flower", "flow", "flight"]),
            "fl"
        );
        assert_eq!(longest_common_prefix(vec!["dog", "racecar", "car"]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["a"]), "a".to_string());
        assert_eq!(longest_common_prefix(vec![""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["a", "a"]), "a".to_string());
        assert_eq!(longest_common_prefix(vec!["a", ""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "a"]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", ""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["a", "a", "a"]), "a".to_string());
        assert_eq!(longest_common_prefix(vec!["a", "a", ""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "a", "a"]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "a", ""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "", "a"]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "", ""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["a", "a", "a", "a"]), "a".to_string());
        assert_eq!(longest_common_prefix(vec!["a", "a", "a", ""]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "a", "a", "a"]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["", "a", "a", ""]), "".to_string());
    }
}
