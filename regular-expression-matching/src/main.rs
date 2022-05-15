#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut s_idx = 0;
        let mut p_idx = 0;
        while s_idx < s.len() && p_idx < p.len() {
            let pattern = Pattern::new(&s[p_idx..std::cmp::min(p_idx + 2, p.len())]);
            p_idx += pattern.forward();
            while s_idx < s.len() {
                let c = s.chars().nth(s_idx).unwrap();
                if pattern.match_with(c) {
                    s_idx += 1;
                    if !pattern.is_any() {
                        break;
                    }
                } else {
                    if pattern.is_any() {
                        break;
                    }
                    return false;
                }
            }
        }
        if s_idx < s.len() || p_idx < p.len() {
            return false;
        }
        true
    }
}

struct Pattern {
    pub c: char,
    is_any: bool,
}

impl Pattern {
    pub fn new(s: &str) -> Self {
        if s.len() == 1 {
            let c = s.chars().nth(0).unwrap();
            return Pattern { c, is_any: false };
        }
        let c = s.chars().nth(0).unwrap();
        let option = s.chars().nth(1).unwrap();
        return Pattern {
            c,
            is_any: option == '*',
        };
    }

    fn is_wild(&self) -> bool {
        self.c == '.'
    }

    pub fn is_any(&self) -> bool {
        self.is_any
    }

    pub fn forward(&self) -> usize {
        if self.is_any {
            return 2;
        } else {
            return 1;
        }
    }

    pub fn match_with(&self, c: char) -> bool {
        if self.is_wild() {
            return true;
        }
        c == self.c
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
