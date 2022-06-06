#![allow(dead_code)]

use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nums = HashSet::<i32>::from_iter(nums.into_iter())
            .into_iter()
            .collect::<Vec<i32>>();
        nums.sort_unstable();
        if nums.len() < 3 {
            return nums.pop().unwrap();
        }
        nums.pop();
        nums.pop();
        nums.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::third_max(vec![3, 2, 1]), 1
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::third_max(vec![1, 2]), 2
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::third_max(vec![2, 2, 3, 1]), 1
        );
    }
}

fn main() {
    println!("leetcode template");
}
