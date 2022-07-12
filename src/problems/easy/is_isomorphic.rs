pub fn is_isomorphic(s: String, t: String) -> bool {
    use std::collections::HashSet;
    let unique_mappings = s.chars().zip(t.chars()).collect::<HashSet<_>>();

    let pre_image = unique_mappings
        .iter()
        .map(|(a, _)| a)
        .collect::<HashSet<_>>();
    let post_image = unique_mappings
        .iter()
        .map(|(_, b)| b)
        .collect::<HashSet<_>>();

    pre_image.len() == unique_mappings.len() && post_image.len() == unique_mappings.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
        assert_eq!(is_isomorphic("ab".to_string(), "aa".to_string()), false);
        assert_eq!(
            is_isomorphic("bbbaaaba".to_string(), "aaabbbba".to_string()),
            false
        );
    }
}
