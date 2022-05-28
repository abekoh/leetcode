#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        Solution::valid_palindrome_with_skip(&s.as_bytes(), true)
    }

    fn valid_palindrome_with_skip(chars: &[u8], can_skip: bool) -> bool {
        if chars.len() < 2 {
            return true;
        }
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i <= j {
            if chars[i] == chars[j] {
                i += 1;
                j -= 1;
                continue;
            }
            if !can_skip {
                return false;
            }
            return vec![
                Solution::valid_palindrome_with_skip(&chars[i..=j - 1], false),
                Solution::valid_palindrome_with_skip(&chars[i + 1..=j], false),
            ]
            .iter()
            .any(|&x| x);
        }
        true
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
