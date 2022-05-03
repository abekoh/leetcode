#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut first: i32 = 0;
        let mut last: i32 = nums.len() as i32 - 1;
        while first <= last {
            let cur = (last - first) / 2 + first;
            if let Some(n) = nums.get(cur as usize) {
                if *n == target {
                    return cur as i32;
                }
                if *n > target {
                    last = cur - 1;
                } else {
                    first = cur + 1;
                }
            } else {
                break;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::search(vec![5], 5), 0);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }
}

fn main() {
    println!("leetcode template");
}
