#![allow(dead_code)]

struct Solution {}

impl Solution {
    // https://leetcode.com/problems/reverse-words-in-a-string-iii/discuss/1640310/Rust-0ms-2.2MB
    pub fn reverse_words(s: String) -> String {
        s.chars()
            .rev()
            .collect::<String>()
            .split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn reverse_words_own(s: String) -> String {
        s.split(' ')
            .map(|x| x.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod reverse_words_in_a_string_iii_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse_words(String::from("Let's take LeetCode contest")),
            String::from("s'teL ekat edoCteeL tsetnoc")
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse_words(String::from("God Ding")),
            String::from("doG gniD")
        )
    }
}

fn main() {
    println!("leetcode template");
}
