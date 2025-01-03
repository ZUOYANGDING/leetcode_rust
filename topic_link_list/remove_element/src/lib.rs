//Definition for singly-linked list.
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut prev = Box::new(ListNode::new(-1));
        prev.next = head;
        let mut cur = prev.as_mut();
        while let Some(next) = cur.next.take() {
            if next.val == val {
                cur.next = next.next;
            } else {
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap();
            }
        }
        return prev.next;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let list = vec![1, 2, 6, 3, 4, 5, 6];
        let mut dummy_node = Box::new(ListNode::new(-1));
        let mut cur_node = dummy_node.as_mut();
        for v in list.into_iter() {
            cur_node.next = Some(Box::new(ListNode::new(v)));
            cur_node = cur_node.next.as_mut().unwrap();
        }
        let val = 6;
        let mut head = Solution::remove_elements(dummy_node.next, val);
        assert!(head.is_some());
        let mut num = 1;

        while let Some(cur) = head.take() {
            assert_eq!(cur.val, num);
            head = cur.next;
            num += 1;
        }
    }
}
