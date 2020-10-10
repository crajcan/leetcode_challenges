pub fn add_to_each(arr: Vec<Vec<i32>>, x: i32) -> Vec<Vec<i32>> {
    arr.iter().map(|sub_array| [vec![x], sub_array.to_vec()].concat()).collect()
}

pub fn powerset(arr: Vec<i32>) -> Vec<Vec<i32>> {
    match arr.split_last() {
        None => { println!("No split.last");
                [].to_vec() },
        Some((x, xs)) => [
            powerset(xs.to_vec()),
            add_to_each(powerset(xs.to_vec()), *x)
        ].concat()
    }
}

pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    (0..arr.len() - 2).fold(0, |count, i| {
        if ((arr[i] - arr[i + 1]).abs() <= a)
            && ((arr[i + 1] - arr[i + 2]).abs() <= b)
            && ((arr[i] - arr[i + 2]).abs() <= c)
        {
/*
            println!(
                "TRUE: arr[i]: {}, arr[i + 1]: {}, arr[i + 2]: {}",
                arr[i],
                arr[i + 1],
                arr[i + 2]
            );
*/
            count + 1
        } else {
/*
            println!(
                "FALSE: arr[i]: {}, arr[i + 1]: {}, arr[i + 2]: {}",
                arr[i],
                arr[i + 1],
                arr[i + 2]
            );
*/
            count
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_to_each() {
        let empty = Vec::new();

        assert_eq!(add_to_each(empty.clone(), 42), empty.clone());
        assert_eq!(add_to_each(vec![vec![]], 42), vec![vec![42]]);
        assert_eq!(
            add_to_each(vec![vec![], vec![3], vec![1, 3]], 42),
            vec![vec![42], vec![42, 3], vec![42, 1, 3]]
        );
    }

    #[test]
    fn test_powerset() {
        assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(powerset(vec![1, 2]), vec![vec![], vec![2], vec![1], vec![1, 2]]);
        assert_eq!(
            powerset(vec![1, 2, 3]),
            vec![vec![], vec![3], vec![2], vec![1], vec![1, 3], vec![1, 2], vec![1, 2, 3]]
        );
    }

    #[test]
    fn test_count_good_triplets() {
        assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
}
