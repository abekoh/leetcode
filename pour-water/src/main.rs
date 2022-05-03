#![allow(dead_code)]

struct Position {
    pub x: i32,
    pub y: i32,
}

struct Solution {}

impl Solution {
    pub fn pour_water(heights: Vec<i32>, volume: i32, k: i32) -> Vec<i32> {
        let mut heights = heights;
        for _ in 0..volume {
            let mut pos = Position {
                x: k,
                y: heights[k as usize] + 1,
            };
            'outer: loop {
                for i in (0..pos.x).rev() {
                    match pos.y - heights[i as usize] {
                        x if x < 1 => {
                            break;
                        }
                        x if x == 1 => {
                            continue;
                        }
                        x if x > 1 => {
                            pos.x = i;
                            pos.y -= x - 1;
                            continue 'outer;
                        }
                        _ => unreachable!(),
                    }
                }
                for i in (pos.x as usize + 1)..heights.len() {
                    match pos.y - heights[i as usize] {
                        x if x < 1 => {
                            break;
                        }
                        x if x == 1 => {
                            continue;
                        }
                        x if x > 1 => {
                            pos.x = i as i32;
                            pos.y -= x - 1;
                            continue 'outer;
                        }
                        _ => unreachable!(),
                    }
                }
                heights[pos.x as usize] += 1;
                break;
            }
        }
        return heights;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::pour_water(vec![2, 1, 1, 2, 1, 2, 2], 4, 3),
            vec![2, 2, 2, 3, 2, 2, 2]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::pour_water(vec![1, 2, 3, 4], 2, 2),
            vec![2, 3, 3, 4]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::pour_water(vec![3, 1, 3], 5, 1), vec![4, 4, 4]);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::pour_water(vec![13, 7, 9, 6, 4, 4, 4, 10, 15, 9], 7, 1),
            vec![13, 9, 9, 6, 6, 6, 5, 10, 15, 9]
        );
    }
}

fn main() {
    println!("leetcode template");
}
