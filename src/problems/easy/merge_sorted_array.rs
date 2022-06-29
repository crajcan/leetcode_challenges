pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    use std::cmp::Ordering::Greater;

    let mut where_to_write = m + n - 1;
    let mut m0 = m - 1;
    let mut n0 = n - 1;

    while where_to_write >= 0 {
        match (m0, n0) {
            (-1, -1) => panic!("should've returned by now"),
            (-1, b) => {
                nums1[where_to_write as usize] = nums2[b as usize];
                n0 -= 1;
            }
            (a, -1) => {
                nums1[where_to_write as usize] = nums1[a as usize];
                m0 -= 1;
            }
            (_, _) => match nums1[m0 as usize].cmp(&nums2[n0 as usize]) {
                Greater => {
                    nums1[where_to_write as usize] = nums1[m0 as usize];
                    m0 -= 1;
                }
                _ => {
                    nums1[where_to_write as usize] = nums2[n0 as usize];
                    n0 -= 1;
                }
            },
        }
        where_to_write = where_to_write - 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
