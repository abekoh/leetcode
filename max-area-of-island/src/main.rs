#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                max = std::cmp::max(max, Self::area(i, j, &grid, &mut seen));
            }
        }
        max
    }

    fn area(x: usize, y: usize, grid: &Vec<Vec<i32>>, seen: &mut Vec<Vec<bool>>) -> i32 {
        if seen[x][y] || grid[x][y] == 0 {
            return 0;
        }
        seen[x][y] = true;
        let mut sum = 1;
        if x > 0 {
            println!("search -x");
            sum += Solution::area(x - 1, y, grid, seen);
        }
        if x < grid.len() - 1 {
            sum += Solution::area(x + 1, y, grid, seen);
        }
        if y > 0 {
            sum += Solution::area(x, y - 1, grid, seen);
        }
        if y < grid[0].len() - 1 {
            sum += Solution::area(x, y + 1, grid, seen);
        }
        sum
    }
}

#[cfg(test)]
mod max_area_of_island_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0],]),
            0
        );
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::max_area_of_island(vec![vec![0], vec![0]]), 0);
    }
    #[test]
    fn example_4() {
        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 1], vec![1, 1]]),
            3
        );
    }
}

fn main() {
    println!("leetcode template");
}
