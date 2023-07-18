pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res = vec![];
    if nums.len() == 0 {
        return res;
    }

    let mut last = nums[0];
    let mut this_range = (nums[0], None);
    for i in 1..nums.len() {
        if nums[i] > last + 1 {
            res.push(range_to_string(this_range));

            this_range = (nums[i], None);
        } else if nums[i] == last + 1 {
            this_range = (this_range.0, Some(nums[i]))
        }

        last = nums[i]; 
    }

    res.push(range_to_string(this_range));

    res 
}

pub fn range_to_string(range: (i32, Option<i32>)) -> String {
    match range {
        (left, None) => left.to_string(),
        (left, Some(right)) => {
            format!("{}->{}", left, right)
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_summary_ranges() {
        let nums = vec![0,1,2,4,5,7];
        let res = super::summary_ranges(nums);
        assert_eq!(res, vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]);

        let nums = vec![0,2,3,4,6,8,9];
        let res = super::summary_ranges(nums);
        assert_eq!(res, vec!["0".to_string(), "2->4".to_string(), "6".to_string(), "8->9".to_string()]);

        let nums = vec![];
        let res = super::summary_ranges(nums);
        assert_eq!(res, Vec::<String>::new());

        let nums = vec![-1];
        let res = super::summary_ranges(nums);
        assert_eq!(res, vec!["-1".to_string()]);

        let nums = vec![0];
        let res = super::summary_ranges(nums);
        assert_eq!(res, vec!["0".to_string()]);

        let nums = vec![1,3];
        let res = super::summary_ranges(nums);
        assert_eq!(res, vec!["1".to_string(), "3".to_string()]);
    }
}