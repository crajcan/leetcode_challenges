fn equal_here(small: &[u8], big: &[u8]) -> bool {
    println!("small: {:?}, big: {:?}", small, big);
    if big.len() < small.len() {
        return false;
    }
    for i in 0..small.len() {
        if small[i] != big[i] {
            return false;
        }
    }
    true
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle == "" {
        return 0;
    }

    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    for i in 0..haystack.len() {
        if equal_here(&needle, &haystack[i..]) {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(-1, str_str("llll".to_string(), "5".to_string()));
        assert_eq!(2, str_str("hello world".to_string(), "ll".to_string()));
    }
}
