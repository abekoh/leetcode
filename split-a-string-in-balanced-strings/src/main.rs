#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut stack = Stack::new();
        let mut res = 0;
        for c in s.chars() {
            if stack.challenge(c) {
                res += 1
            }
        }
        res
    }
}

struct Stack {
    vec: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            vec: vec![],
        }
    }

    pub fn challenge(&mut self, c: char) -> bool {
        if self.vec.is_empty() {
            self.vec.push(Self::convert(c));
            return false;
        }
        if *self.vec.last().unwrap() == c {
            self.vec.pop();
            return self.vec.is_empty();
        } else {
            self.vec.push(Self::convert(c));
            return false;
        }
    }

    fn convert(c: char) -> char {
        return match c {
            'R' => 'L',
            'L' => 'R',
            _ => unreachable!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::balanced_string_split(String::from("RLRRLLRLRL")), 4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::balanced_string_split(String::from("RLLLLRRRLR")), 3
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::balanced_string_split(String::from("LLLLRRRR")), 1
        );
    }
}

fn main() {
    println!("leetcode template");
}
