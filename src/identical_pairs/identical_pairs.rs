use std::collections::HashMap;

fn factorial(n: i32) -> i32 {
    match n {
        0 => 1,
        _ => n * factorial(n-1)
    }
}

fn pairs(n: i32) -> i32 {
    if n < 2 {
        return 0
    } 

    factorial(n) / (2 * factorial(n - 2))
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {  
   let mut frequencies = HashMap::new();

    for num in nums {
        match frequencies.get(&num) {
            Some(frequency) => {
                                   let new_freq = frequency + 1;
                                   frequencies.insert(num, new_freq)
                               },
            None => frequencies.insert(num, 1)
        };
    }; 

    frequencies.values().map(|freq| pairs(*freq)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn test_pairs() {
        assert_eq!(pairs(1), 0);
        assert_eq!(pairs(2), 1);
        assert_eq!(pairs(3), 3);
        assert_eq!(pairs(4), 6);
    }

    #[test]
    fn test_num_identical_pairs() {
      assert_eq!(num_identical_pairs(vec![1]), 0); 
      assert_eq!(num_identical_pairs(vec![1, 1]), 1); 
      assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0); 
      assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4); 
      assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6); 
    }
}
