struct Solution01;

impl Solution01 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n) in nums.iter().enumerate() {
            for (j, m) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                if *m == target - *n {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution01::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution01::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution01::two_sum(vec![3, 3], 6));
}
