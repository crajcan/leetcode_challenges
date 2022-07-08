pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    match k {
        0 => vec![],
        1 => (1..=n).map(|e| vec![e]).collect::<Vec<Vec<i32>>>(),
        _ => {
            let mut combinations = vec![];

            for i in (2..=n).rev() {
                let not_including_i_or_above = combine(i - 1, k - 1);

                for subset in not_including_i_or_above {
                    let mut this_combo = vec![i];

                    this_combo.append(&mut subset.clone());

                    combinations.push(this_combo);
                }
            }

            combinations
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        assert_eq!(
            combine(4, 2),
            vec![
                vec![4, 1],
                vec![4, 2],
                vec![4, 3],
                vec![3, 1],
                vec![3, 2],
                vec![2, 1]
            ]
        );
        assert_eq!(
            combine(4, 3),
            vec![vec![4, 3, 1], vec![4, 3, 2], vec![4, 2, 1], vec![3, 2, 1]]
        );
    }
}
