pub fn smallest_larger_than(c: char, s: &str) -> Option<(usize, char)> {
    let mut result: Option<(usize, char)> = None;

    for (i, x) in s.chars().enumerate() {
        result = match result {
            None => {
                if x > c {
                    Some((i, x))
                } else {
                    None
                }
            }
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

pub fn largest_smaller_than(c: char, s: &str) -> Option<(usize, char)> {
    let mut result: Option<(usize, char)> = None;

    for (i, x) in s.chars().enumerate() {
        result = match result {
            None => {
                if x < c {
                    Some((i, x))
                } else {
                    None
                }
            }
            Some((result_index, result_char)) => {
                if x > result_char && x < c {
                    Some((i, x))
                } else {
                    Some((result_index, result_char))
                }
            }
        }
    }

    result
}

pub fn sort_string(mut s: String) -> String {
    let mut result: String = "".to_string();

    loop {
        match smallest_larger_than('`', &s) {
            None => break,
            Some((i, min)) => {
                s.remove(i);
                result.push(min)
            }
        }

        while let Some((i, min)) = smallest_larger_than(result.chars().last().unwrap(), &s) {
            s.remove(i);
            result.push(min)
        }

        match largest_smaller_than('{', &s) {
            None => break,
            Some((i, max)) => {
                s.remove(i);
                result.push(max)
            }
        }

        while let Some((i, max)) = largest_smaller_than(result.chars().last().unwrap(), &s) {
            s.remove(i);
            result.push(max)
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallest_larger_than() {
        assert_eq!(smallest_larger_than('c', "abc"), None);
        assert_eq!(smallest_larger_than('c', "abcde"), Some((3, 'd')));
        assert_eq!(smallest_larger_than('c', "adcbe"), Some((1, 'd')));
    }

    #[test]
    fn test_largest_smaller_than() {
        assert_eq!(largest_smaller_than('a', "abc"), None);
        assert_eq!(largest_smaller_than('c', "abcde"), Some((1, 'b')));
        assert_eq!(largest_smaller_than('c', "adcbe"), Some((3, 'b')));
    }

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
}
