pub fn third_max(nums: Vec<i32>) -> i32 {
    use std::collections::{BinaryHeap};

    let mut maxes = BinaryHeap::new();
    for num in nums {
        maxes.push(num);
    } 

    let mut top_three = vec![];

    while top_three.len() < 3 {
        if let Some(next_max) = maxes.pop() {
            if !top_three.contains(&next_max) {
                top_three.push(next_max);
            }
        } else {
            break;
        }
    }

    if top_three.len() < 3 {
        top_three[0]
    } else {
        top_three[2]
    }
}

#[cfg(test)]
mod test {
    fn test_third_max_number() {
        assert_eq!(super::third_max(vec![3, 2, 1]), 1);
        assert_eq!(super::third_max(vec![1, 2]), 2);
        assert_eq!(super::third_max(vec![2, 2, 3, 1]), 1);
    }
}
