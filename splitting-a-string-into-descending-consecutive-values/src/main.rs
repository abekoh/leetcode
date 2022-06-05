#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn split_string(s: String) -> bool {
        if Self::to_int(&s) == 0 {
            return false;
        }
        for i in 0..s.len() {
            let prev = Self::to_int(&s[(s.len() - i - 1)..s.len()]);
            if Self::split_string_temp(&s[..(s.len() - i - 1)], prev) {
                return true;
            }
        }
        false
    }

    fn split_string_temp(s: &str, prev: i128) -> bool {
        if Self::to_int(s) == 0 {
            return false;
        }
        for i in 0..s.len() {
            let target = Self::to_int(&s[(s.len() - i - 1)..s.len()]);
            if target - prev == 1 {
                let sl = &s[..(s.len()) - i - 1];
                if sl.len() == 0 {
                    return true;
                }
                if Self::split_string_temp(sl, target) {
                    return true;
                }
            }
        }
        false
    }

    fn to_int(s: &str) -> i128 {
        s.parse::<i128>().unwrap_or(0)
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
}

fn main() {
    println!("leetcode template");
}
