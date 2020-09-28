use std::collections::HashMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {  
    let mut frequencies = HashMap::new();

    for num in nums {
        match frequencies.get(&num) {
            Some((frequency, total)) => {
                let new_freq =  frequency + 1;
                let new_total = frequency + total;
                frequencies.insert(num, (new_freq, new_total))
            },
            None => frequencies.insert(num, (1, 0))
        };
    }; 

    frequencies.values().map(|tuple| tuple.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(num_identical_pairs(vec![1]), 0); 
        assert_eq!(num_identical_pairs(vec![1, 1]), 1); 
        assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0); 
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4); 
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6); 
        assert_eq!(num_identical_pairs(
          vec![6,5,1,5,7,7,9,1,5,7,1,6,10,9,7,4,1,8,7,1,1,8,6,4,7,4,10,5,3,9,10,1,9,5,5,4,1,7,4,2,9,2,6,6,4,2,10,3,5,3,6,4,7,4,6,4,4,6,3,4,10,1,10,6,10,4,9,6,6,4,8,6,9,5,4]),
          303); 
    }
}
