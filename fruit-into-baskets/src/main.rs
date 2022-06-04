#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut map = vec![0; 40001];
        let mut cnt = 0;
        let mut res = 0;
        let mut l = 0;
        for r in 0..fruits.len() {
            if map[fruits[r] as usize] == 0 {
                cnt += 1;
            }
            map[fruits[r] as usize] += 1;
            while cnt > 2 {
                map[fruits[l] as usize] -= 1;
                if map[fruits[l] as usize] == 0 {
                    cnt -= 1;
                }
                l += 1;
            }
            res = std::cmp::max(res, r as i32 - l as i32 + 1);
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
}

fn main() {
    println!("leetcode template");
}
