#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut bottles = num_bottles;
        let mut sum = num_bottles;
        while bottles >= num_exchange {
            let changed = bottles / num_exchange;
            sum += changed;
            bottles = bottles % num_exchange + changed;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}

fn main() {
    println!("leetcode template");
}
