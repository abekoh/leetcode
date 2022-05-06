#![allow(dead_code)]

use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur = nums[0];
        let mut max = nums[0];
        for n in &nums[1..nums.len()] {
            cur = cmp::max(*n, *n+cur);
            max = cmp::max(max, cur);
        }
        max
    }

    pub fn max_sub_array_old(nums: Vec<i32>) -> i32 {
        let mut sum = i32::MIN;
        for n in 1..=nums.len() {
            for i in 0..=(nums.len() - n) {
                sum = i32::max(sum, nums[i..(i + n)].iter().sum());
            }
        }
        sum
    }
}

#[cfg(test)]
mod maximum_subarray_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}

fn main() {
    println!("leetcode template");
}
