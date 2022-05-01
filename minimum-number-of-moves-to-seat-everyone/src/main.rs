#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats.clone();
        seats.sort();
        let mut students = students.clone();
        students.sort();
        let mut result = 0;
        for i in 0..seats.len() {
            result += (seats[i] - students[i]).abs();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exapmle_1() {
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }
    #[test]
    fn exapmle_2() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
            7
        );
    }
    #[test]
    fn exapmle_3() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
            4
        );
    }
}

fn main() {
    println!("leetcode template");
}
