fn subarrays(arr: &[i32]) -> Vec<&[i32]> {


    vec![&arr[0..1], &arr[2..3]]
}

/*
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
   arr[1] 
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarrays() {
        let arr = vec![1,2,3,4,5];
        let res = subarrays(arr.as_slice());

        println!("res: {:?}", res);
/*
        assert_eq!(subarrays(vec![1, 2]), vec![vec![1], vec![2]]);
        assert_eq!(
            subarrays(vec![10, 11, 12]),
            vec![vec![10], vec![11], vec![12], vec![10, 11, 12]]
        );
        let arr = subarrays(vec![1, 4, 2, 5, 3]);
        let slice = &[0..2];

        assert_eq!(
            arr,
            vec![
                &arr[0],
                &arr[1],
                &arr[2],
                &arr[3],
                &arr[4],
                &arr[0..2],
                &arr[1..3],
                &arr[2..4],
                &[0..4]
            ]
        );
*/
    }
/*
    #[test]
    fn test_sub_odd_legnth_subarrays() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    }
*/
}
