pub fn smallest_larger_than(c: char, s: String) -> Option<(usize, char)> {
    let mut result: Option<(usize, char)> = None;

    for (i, x) in s.chars().enumerate() {
        result = match result {
            None => {
                if x > c {
                    Some((i, x))
                } else {
                    None
                }
            },
            Some((result_index, result_char)) => {
                if x < result_char && x > c {
                    Some((i, x))
                } else {
                    Some((result_index, result_char))
                }
            }
        }
    }

    result
}

pub fn largest_smaller_than(c: char, s: String) -> Option<(usize, char)> {
    Some((0, 'a'))
}

/*
pub fn sort_string(s: String) -> String {
    "rat".to_string()
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallest_larger_than() {
        assert_eq!(smallest_larger_than('c', "abc".to_string()), None);
        assert_eq!(smallest_larger_than('c', "abcde".to_string()), Some((3, 'd')));
        assert_eq!(smallest_larger_than('c', "adcbe".to_string()), Some((1, 'd')));
    }

    #[test]
    fn test_largest_smaller_than() {
        assert_eq!(largest_smaller_than('a', "abc".to_string()), None);
        assert_eq!(largest_smaller_than('c', "abcde".to_string()), Some((1, 'b')));
        assert_eq!(largest_smaller_than('c', "adcbe".to_string()), Some((3, 'b')));
    }
/*
    #[test]
    fn test_sort_string() {
        assert_eq!(sort_string("rat".to_string()), "art".to_string());
        assert_eq!(sort_string("spo".to_string()), "ops".to_string());
        assert_eq!(sort_string("ggggggg".to_string()), "ggggggg".to_string());
        assert_eq!(sort_string("leetcode".to_string()), "cdelotee".to_string());
        assert_eq!(
            sort_string("aaaabbbbcccc".to_string()),
            "abccbaabccba".to_string()
        );
    }
*/
}
