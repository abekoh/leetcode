#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut max_t = 0;
        let mut max_c = '0';
        for i in 0..release_times.len() {
            let t = if i == 0 {
                release_times[i]
            } else {
                release_times[i] - release_times[i - 1]
            };
            let c = keys_pressed.chars().nth(i).unwrap();
            if t > max_t || (t == max_t && c > max_c) {
                max_t = t;
                max_c = c;
            }
        }
        return max_c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::slowest_key(vec![9, 29, 49, 50], String::from("cbcd")),
            'c'
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::slowest_key(vec![12, 23, 36, 46, 62], String::from("squda")),
            'a'
        );
    }
}

fn main() {
    println!("leetcode template");
}
