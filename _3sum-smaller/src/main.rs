#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn three_sum_smaller(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut nums = nums.clone();
        nums.sort();

        let mut res: i32 = 0;
        let i = 0;
        for k in (2..nums.len()).rev() {
            for j in (1..nums.len() - 1).rev() {
                if k <= j {
                    continue;
                }
                for i in 0..j {
                    if nums[i] + nums[j] + nums[k] < target {
                        res += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::three_sum_smaller(vec![-2, 0, 1, 3], 2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::three_sum_smaller(vec![], 0), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::three_sum_smaller(vec![0], 0), 0);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::three_sum_smaller(vec![1, -2, 2, 1, 0], 1), 4);
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::three_sum_smaller(vec![1, -1, 2, 0, 3, -2], 2), 10);
    }
}

fn main() {
    println!("leetcode template");
}
