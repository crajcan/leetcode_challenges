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
        
        while lo < hi {
            let mid = lo + ((hi - lo) / 2);
            println!("lo: {}, mid: {}, hi: {}", lo, mid, hi);
            
            println!("left, right: {:?}", (self.is_bad_version(mid), self.is_bad_version(mid + 1)));
            match (self.is_bad_version(mid), self.is_bad_version(mid + 1)) {
                (false, true) => return mid + 1,
                (false, false) => lo = mid + 1,
                (true, true) => hi = mid,
                (true, false) => panic!("invalid state")
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
