pub fn add_to_each(arr: Vec<Vec<i32>>, x: i32) -> Vec<Vec<i32>> {
    arr.iter()
        .map(|sub_array| [sub_array.to_vec(), vec![x]].concat())
        .collect()
}

pub fn powerset(arr: &Vec<i32>) -> Vec<Vec<i32>> {
    match arr.split_last() {
        None => vec![vec![]],
        Some((x, xs)) => {
            let powerset = powerset(&xs.to_vec());
            [powerset.clone(), add_to_each(powerset, *x)].concat()
        }
    }
}

fn triplets(arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    arr.iter().fold(vec![], |acc, e| {
        if e.len() == 3 {
            [acc, vec![e.to_vec()]].concat()
        } else {
            acc
        }
    })
}

pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    (triplets(powerset(&arr))).iter().fold(0, |count, triplet| {
        if ((triplet[0] - triplet[1]).abs() <= a)
            && ((triplet[1] - triplet[2]).abs() <= b)
            && ((triplet[0] - triplet[2]).abs() <= c)
        {
            count + 1
        } else {
            count
        }
    })
}

pub fn fast_count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;

    for i in 0..arr.len() - 2 {
        for j in (i + 1)..arr.len() - 1 {
            for k in (j + 1)..arr.len() {
                if ((arr[i] - arr[j]).abs() <= a)
                    && ((arr[j] - arr[k]).abs() <= b)
                    && ((arr[i] - arr[k]).abs() <= c)
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_to_each() {
        let empty = Vec::new();

        assert_eq!(add_to_each(empty.clone(), 42), empty);
        assert_eq!(add_to_each(vec![vec![]], 42), vec![vec![42]]);
        assert_eq!(
            add_to_each(vec![vec![], vec![3], vec![1, 3]], 42),
            vec![vec![42], vec![3, 42], vec![1, 3, 42]]
        );
    }

    #[test]
    fn test_powerset() {
        assert_eq!(powerset(&vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            powerset(&vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
        assert_eq!(
            powerset(&vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_triplets() {
        assert_eq!(
            triplets(vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]),
            vec![vec![1, 2, 3]]
        )
    }

    #[test]
    fn test_count_good_triplets() {
        assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }

    #[test]
    fn test_fast_count_good_triplets() {
        assert_eq!(fast_count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(fast_count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
}
