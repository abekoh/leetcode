#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res: Vec<char> = vec![];
        for (i, c) in strs[0].chars().enumerate() {
            if strs
                .iter()
                .filter(|s| s.len() > i && s.chars().nth(i).unwrap() == c)
                .count()
                != strs.len()
            {
                break;
            }
            res.push(c);
        }
        res.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );
    }
}

fn main() {
    println!("leetcode template");
}
