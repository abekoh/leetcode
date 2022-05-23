#![allow(dead_code)]

use std::collections::HashMap;

use rand::prelude::*;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.list.len());
        self.list.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx) = self.map.get(&val) {
            let idx = *idx;
            let last_element = self.list[self.list.len() - 1];
            self.list[idx] = last_element.clone();
            self.map.insert(last_element, idx);
            self.list.pop();
            self.map.remove(&val);
            return true;
        }
        false
    }

    fn get_random(&mut self) -> i32 {
        *self.list.iter().choose(&mut self.rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut randomized_set = RandomizedSet::new();
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
