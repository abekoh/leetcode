#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn three_sum_smaller(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        if nums.len() < 3 {
            return 0;
        }
        let mut i = 0;
        let mut j = 1;
        let mut k = 2;
        let mut phase = 0;
        let mut revert = false;
        let mut res = 0;
        while phase <= 2 {
            if nums[i] + nums[j] + nums[k] < target {
                if !revert {
                    res += 1;
                }
                revert = false;
                match phase {
                    0 => {
                        if k < nums.len() - 1 {
                            k += 1;
                        } else {
                            revert = true;
                            phase += 1;
                        }
                    }
                    1 => {
                        if k - j > 1 {
                            j += 1;
                        } else {
                            revert = true;
                            phase += 1
                        }
                    }
                    2 => {
                        if j - i > 1 {
                            i += 1;
                        } else {
                            revert = true;
                            phase += 1;
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                }
                continue;
            } else {
                revert = true;
                match phase {
                    0 => {
                        k -= 1;
                    }
                    1 => {
                        j -= 1;
                    }
                    2 => {
                        i -= 1;
                    }
                    _ => {
                        unreachable!();
                    }
                }
                phase += 1;
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
}

fn main() {
    println!("leetcode template");
}
