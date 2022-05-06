#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut str = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut str);
        assert_eq!(str, vec!['o', 'l', 'l', 'e', 'h']);
    }
    #[test]
    fn example_2() {
        let mut str = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut str);
        assert_eq!(str, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}

fn main() {
    println!("leetcode template");
}
