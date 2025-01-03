struct Solution;
#[derive(Debug, Clone)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(-1));
        dummy_head.next = head;

        let mut fast_pointer = &dummy_head.clone();
        let mut slow_pointer = &mut dummy_head;
        let mut gap = n;
        while gap > 0 {
            fast_pointer = fast_pointer.next.as_ref().unwrap();
            gap -= 1;
        }
        while fast_pointer.next.is_some() {
            slow_pointer = slow_pointer.next.as_mut().unwrap();
            fast_pointer = fast_pointer.next.as_ref().unwrap();
        }
        slow_pointer.next = slow_pointer.next.as_mut().unwrap().next.take();
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn it_works() {
        let mut dummy_head = Box::new(ListNode::new(-1));
        let mut cur = &mut dummy_head;
        for idx in 1..5 {
            let node = Box::new(ListNode::new(idx));
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
        let result = Solution::remove_nth_from_end(dummy_head.next, 2);
        let answer = vec![1, 2, 4];
        let mut idx = 0;
        let mut cur = &result;
        while let Some(node) = cur {
            assert_eq!(node.val, answer[idx]);
            idx += 1;
            cur = &node.next;
        }
    }
}
