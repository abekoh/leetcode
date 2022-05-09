#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut mp = HashMap::new();
        for n in nums1 {
            if let Some(count) = mp.get(&n) {
                mp.insert(n, count.clone() + 1);
            } else {
                mp.insert(n, &1);
            }
        }
        let mut result = vec![];
        for n in nums2 {
            if let Some(count) = mp.get(&n) {
                if *count.clone() > 0 {
                    result.push(n);
                    mp.insert(n, count.clone() - 1);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod maximum_subarray_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
    }

    #[test]
    fn example_2() {
        let actual = Solution::intersect(vec![4, 8, 5], vec![9, 4, 9, 8, 4]);
        assert_eq!(actual.len(), 2);
        assert!(actual.contains(&4));
        assert!(actual.contains(&9));
    }
}

fn main() {
    println!("leetcode template");
}

