#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        false
    }
}

#[cfg(test)]
mod regular_expression_matching_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a")),
            false
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a*")),
            true
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::is_match(String::from("ab"), String::from(".*")),
            true
        );
    }
}

fn main() {
    println!("leetcode template");
}
