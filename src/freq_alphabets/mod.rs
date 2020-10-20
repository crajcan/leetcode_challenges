fn translate(s: &str) -> char {
    (96 + s.parse::<u8>().unwrap()) as char
}
/*
pub fn freq_alphabets(s: String) -> String {
    let i = s.len() - 1;
    let mut result = String::new();
    

    while i >= 0 {
        match s[i] {
            '#' => result.insert(0, translate(&s[i-2..i])),
            _ => result.insert(0, translate(&s[i-1..i]))
       }
    }
}
*/
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transalte() {
        assert_eq!(translate("1"), 'a');
        assert_eq!(translate("9"), 'i');
        assert_eq!(translate("10"), 'j');
        assert_eq!(translate("26"), 'z');
    }

/*
    #[test]
    fn test_freq_alphabets() {
        assert_eq!(freq_alphabets("1236#".to_string()), "acz".to_string());
        assert_eq!(freq_alphabets("10#11#12".to_string()), "jkab".to_string());
        assert_eq!(
            freq_alphabets(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()
            ),
            "abcdefghijklmnopqrstuvwxyz".to_string()
        );
    }
*/
}
