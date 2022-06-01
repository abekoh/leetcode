#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }
}

fn main() {
    println!("leetcode template");
}
