#![allow(dead_code)]

// let m = m as usize;
// let n = n as usize;
// nums1.splice(m..(m + n), nums2.clone());

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let mut i = 0 as usize;
        let mut j = 0 as usize;
        let nums1_copy = nums1.clone();
        for k in 0..(m + n) {
            if j >= n || (i < m && nums1_copy[i] <= nums2[j]) {
                nums1[k] = nums1_copy[i];
                i += 1;
            } else {
                nums1[k] = nums2[j];
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut num1 = vec![1, 2, 3, 0, 0, 0];
        let mut num2 = vec![2, 5, 6];
        Solution::merge(&mut num1, 3, &mut num2, 3);
        assert_eq!(num1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_2() {
        let mut num1 = vec![1];
        let mut num2 = vec![];
        Solution::merge(&mut num1, 1, &mut num2, 0);
        assert_eq!(num1, vec![1]);
    }

    #[test]
    fn example_3() {
        let mut num1 = vec![0];
        let mut num2 = vec![1];
        Solution::merge(&mut num1, 0, &mut num2, 1);
        assert_eq!(num1, vec![1]);
    }

    #[test]
    fn example_4() {
        let mut num1 = vec![1, 0];
        let mut num2 = vec![2];
        Solution::merge(&mut num1, 1, &mut num2, 1);
        assert_eq!(num1, vec![1, 2]);
    }
}

fn main() {
    println!("leetcode template");
}
