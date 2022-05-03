#![allow(dead_code)]

struct Solution {
    first_bad_version: i32,
}

impl Solution {
    pub fn new(v: i32) -> Self {
        Solution {
            first_bad_version: v,
        }
    }

    // The API isBadVersion is defined for you.
    // isBadVersion(version:i32)-> bool;
    // to call it use self.isBadVersion(version)
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        if version >= self.first_bad_version {
            return true;
        }
        false
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut first = 1;
        let mut last = n;
        while first <= last {
            let cur = (last - first) / 2 + first;
            if self.isBadVersion(cur) {
                last = cur - 1;
            } else {
                first = cur + 1;
            }
        }
        return first;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let sol = Solution::new(4);
        assert_eq!(sol.first_bad_version(5), 4);
    }

    #[test]
    fn example_2() {
        let sol = Solution::new(1702766719);
        assert_eq!(sol.first_bad_version(2126753390), 1702766719);
    }
}

fn main() {
    println!("leetcode template");
}
