use std::collections::HashMap;

fn morse_alphabet() -> HashMap<char, String> {
    [
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
    ]
    .iter()
    .map(|(k, v)| (k.clone(), v.to_string()))
    .collect()
}

fn translate(word: String, alphabet: &HashMap<char, String>) -> String {
    word.chars()
        .map(|c| alphabet.get(&c).unwrap().clone())
        .collect()
}

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let alphabet = &morse_alphabet();

    let mut translated: Vec<String> = words
        .iter()
        .map(|word| translate(word.to_string(), alphabet))
        .collect();

    translated.sort();
    translated.dedup();
    translated.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_translate() {
        let alphabet = &morse_alphabet();

        assert_eq!(
            translate("gin".to_string(), alphabet),
            "--...-.".to_string()
        );
        assert_eq!(
            translate("zen".to_string(), alphabet),
            "--...-.".to_string()
        );
        assert_eq!(
            translate("gig".to_string(), alphabet),
            "--...--.".to_string()
        );
        assert_eq!(
            translate("msg".to_string(), alphabet),
            "--...--.".to_string()
        );
    }

    #[test]
    fn test_unique_morse_representations() {
        assert_eq!(
            unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        );

        assert_eq!(
            unique_morse_representations(vec![
                "gin".to_string(),
                "gig".to_string(),
                "zen".to_string(),
                "msg".to_string()
            ]),
            2
        );
    }
}
