#![allow(dead_code)]
// #![allow(unused)]

struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<i32> = nums.iter().map(|x| {
            x * x
        }).collect();
        nums.sort_unstable();
        nums
    }
}

#[cfg(test)]
mod sorted_squares_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}

fn main() {
    println!("leetcode template");
}
