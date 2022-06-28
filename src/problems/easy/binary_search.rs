pub fn search_helper(nums: &[i32], target: i32, start: i32, end: i32) -> i32 {
    match &nums[(start as usize)..(end as usize)] {
        [] => -1,
        [n] => match *n == target {
            true => start,
            false => -1,
        },
        _ => {
            let center = start + ((end - start) / 2);

            let (left, right) = if target < nums[center as usize] {
                (-1, search_helper(nums, target, start, center))
            } else {
                (search_helper(nums, target, center, end), -1)
            };

            match (left, right) {
                (-1, -1) => -1,
                (x, -1) => x,
                (-1, y) => y,
                (_, _) => panic!("cant be found in both sides"),
            }
        }
    }
}

pub fn iter_search(nums: Vec<i32>, target: i32) -> i32 {
    println!("nums {:?}, target: {}", nums, target);
    let mut lo = 0;
    let mut high = nums.len();

    while lo < high {
        let mid = lo + ((high - lo) / 2);
        println!("lo: {}, mid: {}, high: {}", lo, mid, high);

        match target.cmp(&nums[mid]) {
            std::cmp::Ordering::Greater => {
                lo = mid + 1;
            }
            std::cmp::Ordering::Less => {
                high = mid;
            }
            std::cmp::Ordering::Equal => {
                return mid as i32;
            }
        }
    }

    -1
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    search_helper(&nums[..], target, 0, nums.len() as i32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_iter_search() {
        assert_eq!(iter_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(iter_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
