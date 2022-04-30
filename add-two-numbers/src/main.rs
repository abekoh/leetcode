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

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_with_move_up(l1, l2, false)
    }
    fn add_two_numbers_with_move_up(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        is_move_up: bool,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        } else {
            let val1 = if let Some(ref l) = l1 { l.val } else { 0 };
            let val2 = if let Some(ref l) = l2 { l.val } else { 0 };
            let sum_val = val1 + val2 + if is_move_up { 1 } else { 0 };
            let set_val = sum_val % 10;
            let is_move_up = sum_val >= 10;
            let node = ListNode {
                val: set_val,
                next: Solution::add_two_numbers_with_move_up(
                    if let Some(l) = l1 {
                        l.next
                    } else {
                        Option::None
                    },
                    if let Some(ll) = l2 {
                        ll.next
                    } else {
                        Option::None
                    },
                    is_move_up,
                ),
            };
            Some(Box::from(node))
        }
    }
}
fn main() {
    let l1 = Some(Box::from(ListNode {
        val: 2,
        next: Some(Box::from(ListNode {
            val: 4,
            next: Some(Box::from(ListNode::new(3))),
        })),
    }));
    let l2 = Some(Box::from(ListNode {
        val: 5,
        next: Some(Box::from(ListNode {
            val: 6,
            next: Some(Box::from(ListNode::new(4))),
        })),
    }));
    let res = Solution::add_two_numbers(l1, l2);
    println!("{:?}", res);
    ()
}
