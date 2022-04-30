use std::collections::HashMap;

trait TwoSum {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct Solution01;

impl TwoSum for Solution01 {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
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

struct Solution02;

impl TwoSum for Solution02 {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = mp.get(&(target - n)) {
                return vec![i as i32, *j as i32];
            }
            mp.insert(n, i);
        }
        return vec![];
    }
}

fn main() {
    let mut solutions = Vec::<Box<dyn TwoSum>>::new();
    solutions.push(Box::new(Solution01 {}));
    solutions.push(Box::new(Solution02 {}));

    for sol in solutions.iter() {
        println!("{:?}", sol.two_sum(vec![2, 7, 11, 15], 9));
        println!("{:?}", sol.two_sum(vec![3, 2, 4], 6));
        println!("{:?}", sol.two_sum(vec![3, 3], 6));
    }
}
