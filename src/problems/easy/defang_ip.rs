pub fn defang_i_paddr(address: String) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defang_ip() {
        assert_eq!(defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1");
        assert_eq!(
            defang_i_paddr("255.255.255.255".to_string()),
            "255[.]255[.]255[.]255"
        );
    }
}
