pub fn reverse_bits(mut x: u32) -> u32 {
    (0..32).fold(0, |mut res, _| {
        res *= 2;

        if x % 2 == 1 {
            res ^= 1;
        }

        x /= 2;
        res
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(43261596), 964176192);
        assert_eq!(reverse_bits(4294967293), 3221225471);
    }
}
