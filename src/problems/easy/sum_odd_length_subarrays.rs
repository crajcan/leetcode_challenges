fn subarrays(arr: &[i32]) -> Vec<&[i32]> {
    let mut res: Vec<&[i32]> = vec![];

    for i in 0..arr.len() {
        for j in i..arr.len() {
            res.push(&arr[i..=j]);
        }
    }

    res
}

pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    subarrays(arr.as_slice()).iter().fold(0, |acc, x| {
        if x.len() % 2 == 0 {
            acc
        } else {
            let sum: i32 = x.iter().sum();
            acc + sum
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarrays() {
        let expected: Vec<&[i32]> = vec![];
        assert_eq!(subarrays(&[]), expected);

        let expected: Vec<&[i32]> = vec![&[1]];
        assert_eq!(subarrays(&[1]), expected);

        let expected: Vec<&[i32]> = vec![&[1], &[1, 2], &[1, 2, 3], &[2], &[2, 3], &[3]];
        assert_eq!(subarrays(&[1, 2, 3]), expected);
    }

    #[test]
    fn test_sub_odd_legnth_subarrays() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    }
}
