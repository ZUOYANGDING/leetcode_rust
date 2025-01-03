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

pub struct Solution;
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // suppose 1->2->3->4
        // add as -1->1->2->3->4
        let mut dummy_head = Box::new(ListNode::new(-1));
        dummy_head.next = head;
        let mut cur = dummy_head.as_mut();
        // take next_node pointer = 1, make cur.next = None
        while let Some(mut next_node) = cur.next.take() {
            // take next_next_node pointer = 2 if there exists 2, make next_node.next=None
            if let Some(mut next_next_node) = next_node.next.take() {
                // take the next_next_next_node's pointer, might none, make next_next_node.next=None
                let next_next_next_node = next_next_node.next.take();
                // 1 -> 3
                next_node.next = next_next_next_node;
                // 2 -> 1
                next_next_node.next = Some(next_node);
                // -1 -> 2
                cur.next = Some(next_next_node);
                // move cur to 2's next
                cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                // deal with the last node when # of nodes is odd
                cur.next = Some(next_node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy_head.next
    }
}
#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use super::*;

    #[test]
    fn it_works() {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut head = dummy_head.as_mut();
        for idx in 1..4 {
            let mut node = Box::new(ListNode::new(idx));
            node.deref_mut().next = None;
            head.deref_mut().next = Some(node);
            head = head.next.as_mut().unwrap();
        }
        let mut result = Solution::swap_pairs(dummy_head.next);
        let answer = [2, 1, 3];
        let mut idx = 0;
        while let Some(node) = result.take() {
            assert_eq!(answer[idx], node.val);
            result = node.next;
            idx += 1;
        }
    }
}
