#![allow(dead_code)]
#![allow(unused)]

struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // let mut nums = nums.iter().map(|x| {
        //     x * x
        // })
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "tbd"]
    fn example_1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
    #[test]
    #[ignore = "tbd"]
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
