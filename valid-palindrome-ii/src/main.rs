#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        Solution::valid_palindrome_with_skip(&s, true)
    }

    fn valid_palindrome_with_skip(s: &str, can_skip: bool) -> bool {
        if s.len() < 2 {
            return true;
        }
        if s.chars().nth(0).unwrap() == s.chars().nth(s.len() - 1).unwrap() {
            return Solution::valid_palindrome_with_skip(&s[1..s.len() - 1], can_skip);
        }
        if !can_skip {
            return false;
        }
        vec![
            Solution::valid_palindrome_with_skip(&s[0..s.len() - 1], false),
            Solution::valid_palindrome_with_skip(&s[1..s.len()], false),
        ]
        .iter()
        .any(|&x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::valid_palindrome(String::from("aba")), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::valid_palindrome(String::from("abca")), true);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::valid_palindrome(String::from("abc")), false);
    }
}

fn main() {
    println!("leetcode template");
}
