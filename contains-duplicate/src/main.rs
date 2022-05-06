#![allow(dead_code)]

use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len_nums = nums.len();
        let set = HashSet::<i32>::from_iter(nums);
        set.len() != len_nums
    }
}

#[cfg(test)]
mod contains_duplicate_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}

fn main() {
    println!("leetcode template");
}
