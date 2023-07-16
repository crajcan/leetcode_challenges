pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut maxes = vec![];
    println!("maxes: {:?}", maxes);

    for num in nums {
        println!("maxes: {:?}", maxes);
        println!("maxes.len(): {}", maxes.len());
        println!("num: {}", num);

        if maxes.len() == 0 {
            maxes.push(num);
        } else {
            if maxes.len() < 3 || num > maxes[maxes.len() - 1] {
                insert(&mut maxes, num);
            }
        }
    }

    if maxes.len() < 3 {
        maxes[0] 
    } else {
        maxes[maxes.len() - 1]
    }
}

fn insert(maxes: &mut Vec<i32>, num: i32) {
    for i in 0..maxes.len() {
        if num == maxes[i] {
            return;
        }

        if num > maxes[i] {
            maxes.insert(i, num);
            maxes.truncate(3);
            return;
        }
    }
    
    maxes.push(num) 
}

#[cfg(test)]
mod test {
    fn test_third_max_number() {
        assert_eq!(super::third_max(vec![3, 2, 1]), 1);
        assert_eq!(super::third_max(vec![1, 2]), 2);
        assert_eq!(super::third_max(vec![2, 2, 3, 1]), 1);
    }
}
