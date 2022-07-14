pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut p = nums.len() - 1;
    let mut res = vec![0; nums.len()];

    while start <= end {
        if nums[start] * nums[start] < nums[end] * nums[end] {
            res[p] = nums[end] * nums[end];
            end -= 1;
        } else {
            res[p] = nums[start] * nums[start];
            start += 1;
        }

        if p == 0 {
            break;
        }
        p -= 1
    }

    res
}

pub fn sorted_squares_recursive(nums: Vec<i32>) -> Vec<i32> {
    squares_helper(&nums[..])
}

pub fn squares_helper(nums: &[i32]) -> Vec<i32> {
    use std::cmp::Ordering;

    match nums {
        [] => vec![],
        [a] => vec![a * a],
        _ => {
            let left = nums[0] * nums[0];
            let right = nums[nums.len() - 1] * nums[nums.len() - 1];

            match left.cmp(&right) {
                Ordering::Less => {
                    let mut rest = squares_helper(&nums[..nums.len() - 1]);
                    rest.push(right);
                    rest
                }
                _ => {
                    let mut rest = squares_helper(&nums[1..nums.len()]);
                    rest.push(left);
                    rest
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        let x = [3, 4];
        let y: Vec<_> = [5].iter().chain(&x).copied().collect();
        println!("y: {:?}", y);

        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn test_sorted_squares_recursive() {
        assert_eq!(
            sorted_squares_recursive(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
}
