pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits.clone();
    for i in 0..digits.len() {
        let this_index = digits.len() - 1 - i;
        println!("this_index = {}", this_index);

        match digits[this_index] {
            9 => {
                println!("got here");
                digits[this_index] = 0;
                println!("digits = {:?}", digits);
            }
            _ => {
                digits[this_index] = digits[this_index] + 1;
                return digits;
            }
        }
    }

    if digits[0] == 0 {
        let mut new = vec![1];
        new.append(&mut digits);
        digits = new;
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let digits = vec![1, 2, 3];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 2, 4]);

        let digits = vec![9];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 0]);
    }
}
