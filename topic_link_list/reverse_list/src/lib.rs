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

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut cur = head;
        let mut pre = None;
        while let Some(mut cur_node) = cur.take() {
            cur = cur_node.next;
            cur_node.next = pre;
            pre = Some(cur_node);
        }
        return pre;
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    fn it_works() {
        let mut head = Box::new(ListNode::new(1));
        let mut cur_node: &mut ListNode = head.borrow_mut();
        for n in 2..5 {
            cur_node.next = Some(Box::new(ListNode::new(n)));
            cur_node = cur_node.next.as_mut().unwrap();
        }
        let mut result = Solution::reverse_list(Some(head));
        let mut num = 4;
        while let Some(node) = result.take() {
            assert_eq!(node.val, num);
            result = node.next;
            num -= 1;
        }
    }
}
