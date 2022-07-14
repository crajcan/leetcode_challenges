pub fn my_sqrt(x: i32) -> i32 {
    let mut i = 1;
    while i <= (x / i) {
        i += 1;
    }

    i - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(2), 1);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
    }
}
