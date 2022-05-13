#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let old_color = image[sr][sc];
        let mut image = image.clone();
        if image[sr][sc] != new_color {
            Self::flood_fill_r(&mut image, sr, sc, old_color, new_color);
        }
        image
    }

    fn flood_fill_r(
        image: &mut Vec<Vec<i32>>,
        sr: usize,
        sc: usize,
        old_color: i32,
        new_color: i32,
    ) {
        if image[sr][sc] != old_color {
            return;
        }
        image[sr][sc] = new_color;
        if sr > 0 {
            Self::flood_fill_r(image, sr - 1, sc, old_color, new_color);
        }
        if sr < image.len() - 1 {
            Self::flood_fill_r(image, sr + 1, sc, old_color, new_color);
        }
        if sc > 0 {
            Self::flood_fill_r(image, sr, sc - 1, old_color, new_color);
        }
        if sc < image[0].len() - 1 {
            Self::flood_fill_r(image, sr, sc + 1, old_color, new_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}

fn main() {
    println!("leetcode template");
}
