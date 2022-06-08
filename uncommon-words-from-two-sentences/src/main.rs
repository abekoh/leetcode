#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let map1 = Self::create_map(s1);
        let map2 = Self::create_map(s2);
        let mut res = vec![];
        for k in map1.keys().chain(map2.keys()) {
            let has1 = map1.get(k);
            let has2 = map2.get(k);
            if has1.is_some() && has2.is_some() {
                continue;
            }
            if let Some(t) = has1 {
                if *t > 0 {
                    continue;
                }
            }
            if let Some(t) = has2 {
                if *t > 0 {
                    continue;
                }
            }
            res.push(k.to_string());
        }
        res
    }

    fn create_map(s: String) -> HashMap<String, i32> {
        let mut map = HashMap::<String, i32>::new();
        for word in s.split(' ') {
            if let Some(n) = map.get(word) {
                map.insert(word.to_string(), n + 1);
            } else {
                map.insert(word.to_string(), 0);
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let res = Solution::uncommon_from_sentences(String::from("this apple is sweet"), String::from("this apple is sour"));
        assert_eq!(res.len(), 2);
        assert!(res.contains(&String::from("sweet")));
        assert!(res.contains(&String::from("sour")));
    }

    #[test]
    fn example_2() {
        let res = Solution::uncommon_from_sentences(String::from("apple apple"), String::from("banana"));
        assert_eq!(res.len(), 1);
        assert!(res.contains(&String::from("banana")));
    }
}

fn main() {
    println!("leetcode template");
}
