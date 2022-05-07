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
        let mut fast_cur = &head;
        let mut slow_cur = &head;
        let mut i = 0;
        while let Some(node) = fast_cur {
            if i % 2 == 1 {
                slow_cur = &slow_cur.as_ref().unwrap().next;
            }
            fast_cur = &node.next;
            i +=1;
        }
        slow_cur.clone()
    }

    // https://leetcode.com/problems/middle-of-the-linked-list/discuss/476670/Rust
    pub fn middle_node_another(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = head.clone();
        loop {
            if ptr.is_none() || ptr.as_ref()?.next.is_none() {
                break;
            }
            ptr = ptr.unwrap().next.unwrap().next;
            head = head.unwrap().next;
        }
        head
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
