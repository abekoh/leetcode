#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut res = vec![];
        let mut queue = vec![];
        queue.push(root.unwrap());
        while queue.len() > 0 {
            let elements = queue.iter().map(|x| x.borrow().val).collect::<Vec<i32>>();
            res.push(elements);
            queue = queue
                .iter()
                .map(|x| {
                    let mut r = vec![];
                    if let Some(left) = x.borrow().left.clone() {
                        r.push(left);
                    }
                    if let Some(right) = x.borrow().right.clone() {
                        r.push(right);
                    }
                    return r;
                })
                .flatten()
                .collect();
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let actual = Solution::level_order(Some(Rc::from(RefCell::from(TreeNode {
            val: 3,
            left: Some(Rc::from(RefCell::from(TreeNode::new(9)))),
            right: Some(Rc::from(RefCell::from(TreeNode {
                val: 20,
                left: Some(Rc::from(RefCell::from(TreeNode::new(15)))),
                right: Some(Rc::from(RefCell::from(TreeNode::new(7)))),
            }))),
        }))));
        assert_eq!(actual, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}

fn main() {
    println!("leetcode template");
}
