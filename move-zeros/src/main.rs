#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn sum_two_numbers(x: i32, y: i32) -> i32 {
        x + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::sum_two_numbers(1, 2), 3);
    }
}

fn main() {
    println!("leetcode template");
}

