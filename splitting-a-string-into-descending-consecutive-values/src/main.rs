#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn split_string(s: String) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::split_string(String::from("1234")), false
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::split_string(String::from("050043")), true
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::split_string(String::from("9080701")), false
        );
    }
}

fn main() {
    println!("leetcode template");
}
