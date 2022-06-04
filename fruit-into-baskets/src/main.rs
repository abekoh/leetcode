#![allow(dead_code)]

use std::{cmp::max, collections::HashMap};

struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, i32>::new();
        for f in fruits.clone() {
            match map.get_mut(&f) {
                Some(el) => *el += 1,
                None => {
                    map.insert(f, 1);
                    ()
                }
            }
        }
        println!("{:?}", map);
        println!("{:?}", fruits);
        let mut res = 0;
        for i in 0..(fruits.len() - 1) {
            let first = fruits[i];
            let second = fruits[i + 1];
            let count: i32 = if first == second {
                *map.get(&first).unwrap()
            } else {
                map.get(&first).unwrap() + map.get(&second).unwrap()
            };
            res = max(res, count);
            println!("{}", res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            4
        );
    }
}

fn main() {
    println!("leetcode template");
}
