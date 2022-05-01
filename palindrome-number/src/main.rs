#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }

    fn old(x: i32) -> bool {
        if x >= 0 && x < 10 {
            return true;
        }
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let x_str = x.to_string();
        for i in 0..(x_str.len() / 2) {
            let j = x_str.len() - i - 1;
            if x_str.chars().nth(i).unwrap() != x_str.chars().nth(j).unwrap() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exapmle_1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn exapmle_2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn exapmle_3() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn exapmle_4() {
        assert!(Solution::is_palindrome(2992));
    }

    #[test]
    fn exapmle_5() {
        assert!(Solution::is_palindrome(90709));
    }
}

fn main() {
    println!("leetcode template");
}
