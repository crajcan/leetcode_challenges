pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::HashSet;

    let words = s.split(' ').collect::<Vec<_>>();
    let chars = pattern.chars();

    if words.len() != pattern.len() {
        return false;
    }

    let zipped = chars.zip(words);
    let mapping = zipped.collect::<HashSet<(char, &str)>>();

    let pre_image = mapping.iter().map(|(c, _s)| c).collect::<Vec<_>>();
    let post_image = mapping.iter().map(|(_c, s)| *s).collect::<Vec<_>>();

    let unique_pre_image = pre_image.iter().collect::<HashSet<_>>();
    let unique_post_image = post_image.iter().collect::<HashSet<_>>();

    unique_pre_image.len() == pre_image.len() && unique_post_image.len() == post_image.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_pattern() {
        assert_eq!(
            word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
        assert_eq!(
            word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
        assert_eq!(
            word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
        assert_eq!(
            word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }
}
