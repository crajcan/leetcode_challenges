use std::collections::HashMap;

pub fn triangle_helper(
    triangle: &[Vec<i32>],
    i: usize,
    results: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    match triangle.len() {
        0 => 0,
        1 => triangle[0][0],
        _ => {
            let val = match triangle[triangle.len() - 1].get(i) {
                Some(num) => num,
                None => return 10001,
            };

            let key = (i, triangle.len());

            if results.contains_key(&key) {
                let res = *results.get(&key).unwrap();
                return res;
            }

            let left_total = if i > 0 {
                val + triangle_helper(&triangle[..triangle.len() - 1], i - 1, results)
            } else {
                10001
            };
            let right_total = val + triangle_helper(&triangle[..triangle.len() - 1], i, results);

            let min = std::cmp::min(left_total, right_total);

            results.insert(key, min);

            min
        }
    }
}

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut results = HashMap::new();

    triangle
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, _)| triangle_helper(&triangle[..], i, &mut results))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimum_total() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(minimum_total(triangle), 11);
    }
}
