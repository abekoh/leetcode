#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        for _ in 0..grid.len() + 1 {
            dp.push(vec![1000; grid[0].len() + 1]);
        }
        for i in 1..grid.len() + 1 {
            for j in 1..grid[0].len() + 1 {
                if i == 1 && j == 1 {
                    dp[i][j] = grid[0][0];
                    continue;
                }
                dp[i][j] = std::cmp::min(
                    dp[i - 1][j] + grid[i - 1][j - 1],
                    dp[i][j - 1] + grid[i - 1][j - 1],
                );
            }
        }
        dp[grid.len()][grid[0].len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}

fn main() {
    println!("leetcode template");
}
