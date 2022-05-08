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
    // https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/414563/rust-safe-code-one-pass(has-to-use-clone-to-bypass-borrow-checker)
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);

        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        let next = slow.next.as_mut().unwrap();
        slow.next = next.next.clone();
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from(&vec![1, 2, 3, 4, 5]), 2),
            ListNode::from(&vec![1, 2, 3, 5])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from(&vec![1]), 1),
            ListNode::from(&vec![])
        );
    }
}

fn main() {
    println!("leetcode template");
}
