#![allow(dead_code)]

use std::collections::LinkedList;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = ParenthesesStack::new();
        for c in s.chars() {
            if !stack.push_or_pop(c) {
                return false;
            }
        }
        stack.is_empty()
    }
}

struct ParenthesesStack {
    stack: LinkedList<char>,
}

fn pair(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => unreachable!(),
    }
}

impl ParenthesesStack {
    pub fn new() -> Self {
        ParenthesesStack {
            stack: LinkedList::new(),
        }
    }

    pub fn push_or_pop(&mut self, c: char) -> bool {
        match c {
            '(' | '[' | '{' => self.stack.push_back(c),
            _ => {
                if let Some(back) = self.stack.pop_back() {
                    if back != pair(c) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}

fn main() {
    println!("leetcode template");
}
