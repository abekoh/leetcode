#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut res = 0;
        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                for k in j + 1..arr.len() {
                    if (arr[j] - arr[k]).abs() > b {
                        continue;
                    }
                    if (arr[k] - arr[i]).abs() > c {
                        continue;
                    }
                    res += 1;
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
        assert_eq!(
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1),
            0
        );
    }
}

fn main() {
    println!("leetcode template");
}
