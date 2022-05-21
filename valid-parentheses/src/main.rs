#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}

fn main() {
    println!("leetcode template");
}
