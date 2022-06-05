#![allow(dead_code)]

use std::num::ParseIntError;

struct Solution {}

impl Solution {
    pub fn split_string(s: String) -> bool {
        if s.len() == 0 || Self::parse_int(&s) == 0 {
            return false;
        }
        for i in 0..s.len() {
            let prev = Self::parse_int(&s[(s.len() - i - 1)..s.len()]);
            if Self::helper(&s[..(s.len() - i - 1)], prev) {
                return true;
            }
        }
        false
    }

    fn helper(s: &str, prev: i128) -> bool {
        if s.len() == 0 {
            return false;
        }
        for i in 0..s.len() {
            let current = Self::parse_int(&s[(s.len() - i - 1)..s.len()]);
            if current - prev == 1 {
                let sl = &s[..(s.len()) - i - 1];
                if sl.len() == 0 || Self::helper(sl, current) {
                    return true;
                }
            }
        }
        false
    }

    fn parse_int(s: &str) -> i128 {
        s.parse::<i128>().unwrap()
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

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::split_string(String::from("0090089")), true
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Solution::split_string(String::from("00000")), false
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            Solution::split_string(String::from("10009998")), true
        );
    }

    #[test]
    fn example_7() {
        assert_eq!(
            Solution::split_string(String::from("4771447713")), true
        );
    }

    #[test]
    fn example_8() {
        assert_eq!(
            Solution::split_string(String::from("22759227582275722756")), true
        );
    }

    #[test]
    fn example_9() {
        assert_eq!(
            Solution::split_string(String::from("")), false
        );
    }
}

fn main() {
    println!("leetcode template");
}
