#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() == 1 {
            return;
        }
        let k = k as usize % nums.len();
        let cloned = nums.clone();
        nums.splice(
            0..k,
            cloned[(nums.len() - k)..nums.len() as usize]
                .iter()
                .cloned(),
        );
        nums.splice(k..nums.len(), cloned[0..(nums.len() - k)].iter().cloned());
    }
}

#[cfg(test)]
mod rotate_tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut input, 3);
        assert_eq!(input, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example_2() {
        let mut input = vec![-1, -100, 3, 99];
        Solution::rotate(&mut input, 2);
        assert_eq!(input, vec![3, 99, -1, -100]);
    }
}

fn main() {
    println!("leetcode template");
}
