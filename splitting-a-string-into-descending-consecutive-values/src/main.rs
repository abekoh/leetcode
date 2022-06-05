#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn split_string(s: String) -> bool {
        for i in 1..s.len() {
            let prev = Self::to_int(&s[(s.len() - i)..s.len()]);
            if Self::split_string_temp(&s[..(s.len() - i)], prev) {
                return true;
            }
        }
        false
    }

    fn split_string_temp(s: &str, prev: i32) -> bool {
        for i in 1..s.len() {
            let target = Self::to_int(&s[(s.len() - i)..s.len()]);
            if target - prev == 0 {
                if Self::split_string_temp(&s[..(s.len()) - i], target) {
                    return true;
                }
            }
        }
        false
    }

    fn to_int(s: &str) -> i32 {
        s.parse::<i32>().unwrap()
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
}

fn main() {
    println!("leetcode template");
}
