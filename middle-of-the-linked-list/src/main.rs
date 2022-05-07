#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    #[inline]
    fn from(values: &[i32]) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }
        Some(Box::from(ListNode {
            val: values[0],
            next: Self::from(&values[1..]),
        }))
    }
}

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod middle_of_the_linked_list_tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(
            ListNode::from(&vec![1, 2, 3]),
            Some(Box::from(ListNode {
                val: 1,
                next: Some(Box::from(ListNode {
                    val: 2,
                    next: Some(Box::from(ListNode { val: 3, next: None }))
                }))
            }))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::middle_node(ListNode::from(&vec![1, 2, 3, 4, 5])),
            ListNode::from(&vec![3, 4, 5])
        );
    }
}

fn main() {
    println!("leetcode template");
}
