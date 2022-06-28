struct Solution {
    first_bad_version: usize,
}

impl Solution {
    pub fn is_bad_version(&self, version: i32) -> bool {
        self.first_bad_version <= version as usize
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 0;
        let mut hi = n;

        while lo <= hi {
            let mid = lo + ((hi - lo) / 2);

            let mid_is_bad = self.is_bad_version(mid);
            match mid_is_bad {
                true => {
                    if mid == 1 {
                        return mid;
                    }
                    if !self.is_bad_version(mid - 1) {
                        return mid;
                    }

                    hi = mid;
                }
                false => lo = mid + 1,
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_bad_version() {
        let s = Solution {
            first_bad_version: 4,
        };

        assert_eq!(s.first_bad_version(5), 4);
    }
}
