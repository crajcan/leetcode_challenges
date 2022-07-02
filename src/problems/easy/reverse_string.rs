pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();

    for i in 0..(s.len() / 2) {
        let temp = s[i];
        s[i] = s[len - 1 - i];
        s[len - 1 - i] = temp;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
