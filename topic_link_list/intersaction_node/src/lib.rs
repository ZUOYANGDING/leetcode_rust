use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        Self { val, next: None }
    }
}

struct Solution;

impl Solution {
    fn intersaction_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        // Helper function to calculate the length of a list
        fn get_length(mut head: Option<Rc<RefCell<ListNode>>>) -> i32 {
            let mut length = 0;
            while let Some(node) = head {
                length += 1;
                head = node.borrow().next.clone();
            }
            length
        }

        // Calculate lengths of both lists
        let len_a = get_length(head_a.clone());
        let len_b = get_length(head_b.clone());

        // Align the longer list
        let mut cur_a = head_a.clone();
        let mut cur_b = head_b.clone();

        if len_a > len_b {
            for _ in 0..(len_a - len_b) {
                if let Some(node) = cur_a {
                    cur_a = Rc::clone(&node).borrow().next.clone();
                }
            }
        } else {
            for _ in 0..(len_b - len_a) {
                if let Some(node) = cur_b {
                    cur_b = Rc::clone(&node).borrow().next.clone();
                }
            }
        }

        // Traverse both lists to find the intersection node
        while let (Some(node_a), Some(node_b)) = (cur_a.clone(), cur_b.clone()) {
            // Rc::ptr_eq will compare the T's address wrapped by Rc, here compare the RefCell's address, not the ListNode's address
            // So do not use the Rc::ptr_eq here, even the RefCell of node_a and node_b's commnd node has same RefCell address on heap
            if node_a.deref().as_ptr() == node_b.deref().as_ptr() {
                return Some(node_a);
            }
            // following provide address of RefCell and ListNode
            // if Rc::ptr_eq(&node_a, &node_b) {
            //     println!("get equal a as ListNode: {:p}", node_a.deref().as_ptr());
            //     println!("get equal a as RefCell<ListNode>: {:p}", node_a.deref());
            //     println!("get equal b as ListNode: {:p}", node_b.deref().as_ptr());
            //     println!("get equal b as RefCell<ListNode>: {:p}", node_b.deref());
            //     return Some(node_a);
            // }

            cur_a = Rc::clone(&node_a).borrow().next.clone();
            cur_b = Rc::clone(&node_b).borrow().next.clone();
        }

        None
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    fn build_list(values: &[i32]) -> Option<Rc<RefCell<ListNode>>> {
        let mut head = None;
        for &val in values {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Rc::new(RefCell::new(node)));
        }
        head
    }

    #[test]
    fn test_intersaction() {
        let common = build_list(&[10, 9, 8]);
        let stack_address = common.as_ref().unwrap() as *const _;
        let heap_address = common.as_deref().unwrap().as_ptr();
        let heap_address_of_refcell = common.as_deref().unwrap() as *const _;
        println!("stack_address of pointer ListNode {:p}", stack_address);
        println!("heap_addrsss of ListNode {:p}", heap_address);
        println!(
            "heap_address of RefCell of ListNode: {:p}",
            heap_address_of_refcell
        );
        let common = common.clone();
        let stack_address = common.as_ref().unwrap() as *const _;
        let heap_address = common.as_deref().unwrap().as_ptr();
        let heap_address_of_refcell = common.as_deref().unwrap() as *const _;
        println!("stack_address of pointer ListNode {:p}", stack_address);
        println!("heap_addrsss of ListNode {:p}", heap_address);
        println!(
            "heap_address of RefCell of ListNode: {:p}",
            heap_address_of_refcell
        );
        // Build List A
        let mut head_a = build_list(&[6, 5, 4]);
        if let Some(node) = head_a.as_mut() {
            node.borrow_mut()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .next = common.clone();
        }
        // Build List B
        let mut head_b = build_list(&[6, 2, 4]);
        if let Some(node) = head_b.as_mut() {
            node.borrow_mut()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .next = common.clone();
        }
        println!("{:?}", head_a);
        println!("{:?}", head_b);
        // Find intersection
        let result = Solution::intersaction_node(head_a.clone(), head_b.clone());

        // Print result
        if let Some(node) = result {
            println!("Intersection at node with value: {}", node.borrow().val);
        } else {
            println!("No intersection.");
        }
    }
}
