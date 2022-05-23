#![allow(dead_code)]

struct RandomizedSet {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        unimplemented!();
    }

    fn insert(&self, val: i32) -> bool {
        unimplemented!();
    }

    fn remove(&self, val: i32) -> bool {
        unimplemented!();
    }

    fn get_random(&self) -> i32 {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let randomized_set = RandomizedSet {};
        randomized_set.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
        randomized_set.remove(2); // Returns false as 2 does not exist in the set.
        randomized_set.get_random(); // Inserts 2 to the set, returns true. Set now contains [1,2].
        randomized_set.get_random(); // getRandom() should return either 1 or 2 randomly.
        randomized_set.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
        randomized_set.insert(2); // 2 was already in the set, so return false.
        randomized_set.get_random(); // Since 2 is the only number in the set, getRandom() will always return 2.
        assert_eq!(randomized_set.get_random(), 2);
    }
}

fn main() {
    println!("leetcode template");
}
