pub fn hamming_weight(mut n: u32) -> i32 {
    let mut total = 0;

    while n > 0 {
        if n % 2 == 1 {
            total += 1;
        }

        n = n / 2;
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(0x00000001), 1);
        assert_eq!(hamming_weight(0x00000011), 2);
        assert_eq!(hamming_weight(0x00000111), 3);
        assert_eq!(hamming_weight(0x10000111), 4);
        assert_eq!(hamming_weight(0x11000111), 5);
        assert_eq!(hamming_weight(0x11101011), 6);
        assert_eq!(hamming_weight(0x11111111), 8);
    }
}
