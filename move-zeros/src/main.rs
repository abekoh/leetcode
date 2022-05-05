#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        nums.retain(|&x| x != 0);
        nums.extend(std::iter::repeat(0).take(n - nums.len()));
    }

    pub fn move_zeroes_old(nums: &mut Vec<i32>) {
        let l = nums.len();
        *nums = nums.iter().copied().filter(|x| *x != 0).collect();
        *nums = nums
            .iter()
            .copied()
            .chain([0].repeat(l - nums.len()))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}

fn main() {
    println!("leetcode template");
}
