#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = 0;
        for price in prices {
            if price < min {
                min = price;
            } else if price - min > max {
                max = price - min;
            }
        }
        return max;
    }
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}

fn main() {
    println!("leetcode template");
}
