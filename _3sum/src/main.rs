#![allow(dead_code)]

use std::collections::HashSet;

struct Solution {}

impl Solution {
    // c.f. https://en.wikipedia.org/wiki/3SUM
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut result_set = HashSet::<Vec<i32>>::new();
        for i in 0..=(n - 2) {
            let a = nums[i];
            let mut start = i + 1;
            let mut end = n - 1;
            while start < end {
                let b = nums[start];
                let c = nums[end];
                match a + b + c {
                    0 => {
                        result_set.insert(vec![a, b, c]);
                        start += 1;
                        end -= 1;
                    }
                    1.. => {
                        end -= 1;
                    }
                    _ => {
                        start += 1;
                    }
                }
            }
        }
        result_set.into_iter().collect()
    }

    pub fn three_sum_old(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut result_set = HashSet::<Vec<i32>>::new();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if let Some(n) = nums
                    .iter()
                    .skip(j + 1)
                    .find(|&&x| x == 0 - nums[i] - nums[j])
                {
                    let mut tmp = vec![nums[i], nums[j], *n];
                    tmp.sort_unstable();
                    result_set.insert(tmp);
                }
            }
        }
        result_set.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::three_sum(Vec::<i32>::new()),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<i32>>::new());
    }
}

fn main() {
    println!("leetcode template");
}
