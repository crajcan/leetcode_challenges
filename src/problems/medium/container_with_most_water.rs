pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_vol = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left != right {
        let this_vol = std::cmp::min(height[left],height[right]) * ((right - left) as i32);

        if this_vol > max_vol {
            max_vol = this_vol;
        }

        if height[left] < height[right] {
            left = left + 1;
        } else {
            right = right - 1;
        }
    }

    max_vol 
}

#[cfg(test)]
mod test {
    #[test]
    fn test_max_area() {
        assert_eq!(super::max_area(vec![1,1]), 1);
        assert_eq!(super::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}