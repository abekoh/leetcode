#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut first = 0;
        let mut last = nums.len();
        while first != last {
            let cur = (last - first) / 2 + first;
            if nums[cur] == target {
                return cur as i32;
            } else if nums[cur] > target {
                last = cur;
            } else {
                first = cur + 1;
            }
        }
        first as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}

fn main() {
    println!("leetcode template");
}
