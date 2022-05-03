#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first = 0;
        let mut last = numbers.len() - 1;
        while first < last {
            let n = numbers[first] + numbers[last];
            match n {
                x if x == target => {
                    return vec![first as i32 + 1, last as i32 + 1];
                }
                x if x < target => {
                    first += 1;
                }
                _ => {
                    last -= 1;
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}

fn main() {
    println!("leetcode template");
}
