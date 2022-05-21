#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mp = HashMap::<char, i32>::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut i = 0;
        let mut sum = 0;
        while i < s.len() {
            let cur = s.chars().nth(i);
            let next = s.chars().nth(i + 1);
            if next.is_some() && mp.get(&cur.unwrap()) < mp.get(&next.unwrap()) {
                sum += mp.get(&next.unwrap()).unwrap() - mp.get(&cur.unwrap()).unwrap();
                i += 2;
            } else {
                sum += mp.get(&cur.unwrap()).unwrap();
                i += 1;
            }
        }
        sum
    }
    pub fn roman_to_into_own(s: String) -> i32 {
        let mut sum = 0;
        let mut tmp_add = 0;
        let mut prefix = 'Z';
        for c in s.chars() {
            match c {
                'I' => {
                    sum += tmp_add;
                    tmp_add = 1;
                }
                'V' => {
                    if prefix == 'I' {
                        sum += 4;
                    } else {
                        sum += 5 + tmp_add;
                    }
                    tmp_add = 0;
                }
                'X' => {
                    if prefix == 'I' {
                        sum += 9;
                        tmp_add = 0;
                    } else {
                        sum += tmp_add;
                        tmp_add = 10;
                    }
                }
                'L' => {
                    if prefix == 'X' {
                        sum += 40;
                    } else {
                        sum += 50 + tmp_add;
                    }
                    tmp_add = 0;
                }
                'C' => {
                    if prefix == 'X' {
                        sum += 90;
                        tmp_add = 0;
                    } else {
                        sum += tmp_add;
                        tmp_add = 100;
                    }
                }
                'D' => {
                    if prefix == 'C' {
                        sum += 400;
                    } else {
                        sum += 500 + tmp_add;
                    }
                    tmp_add = 0;
                }
                'M' => {
                    if prefix == 'C' {
                        sum += 900;
                    } else {
                        sum += 1000 + tmp_add;
                    }
                    tmp_add = 0;
                }
                _ => {}
            }
            prefix = c;
        }
        sum + tmp_add
    }
}

#[cfg(test)]
mod roman_to_integer_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::roman_to_int(String::from("DCXXI")), 621);
    }
}

fn main() {
    println!("leetcode template");
}
