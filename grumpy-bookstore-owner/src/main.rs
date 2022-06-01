#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut st = 0;
        let mut mx = 0;
        for i in 0..(customers.len() + 1 - minutes as usize) {
            let mut tmp = 0;
            for j in i..(i + minutes as usize) {
                if grumpy[j] == 1 {
                    tmp += customers[j];
                }
            }
            if tmp > mx {
                st = i;
                mx = tmp;
            }
        }
        let mut res = 0;
        for (i, c) in customers.iter().enumerate() {
            if i >= st && i < st + minutes as usize {
                res += c;
            } else {
                if grumpy[i] == 0 {
                    res += c;
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
