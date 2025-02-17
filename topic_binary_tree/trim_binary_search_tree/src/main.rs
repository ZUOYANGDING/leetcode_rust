/**
 * Leetcode 669
 */
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut cur_node = root.as_ref().unwrap().borrow_mut();
        if cur_node.val < low {
            // trim the cur_node and its left sub_tree
            return Self::trim_bst(cur_node.right.clone(), low, high);
        }
        if cur_node.val > high {
            // trim the cur_node and its right sub_tree
            return Self::trim_bst(cur_node.left.clone(), low, high);
        }
        cur_node.left = Self::trim_bst(cur_node.left.clone(), low, high);
        cur_node.right = Self::trim_bst(cur_node.right.clone(), low, high);
        drop(cur_node);
        root
    }

    fn construct_tree_from_array(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.is_empty() || array[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(array[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut idx = 1; // Start from index 1 since root is already used

        while !queue.is_empty() && idx < array.len() {
            let cur_node = queue.pop_front().unwrap();

            // Process left child
            if idx < array.len() {
                if let Some(val) = array[idx] {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    cur_node.borrow_mut().left = Some(left_child.clone());
                    queue.push_back(left_child);
                }
                idx += 1;
            }

            // Process right child
            if idx < array.len() {
                if let Some(val) = array[idx] {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    cur_node.borrow_mut().right = Some(right_child.clone());
                    queue.push_back(right_child);
                }
                idx += 1;
            }
        }
        Some(root)
    }

    fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let cur_node = root.as_ref().unwrap().clone();
        Self::in_order(cur_node.borrow().left.clone(), ret);
        ret.push(cur_node.borrow().val);
        Self::in_order(cur_node.borrow().right.clone(), ret);
    }
}

fn main() {
    let root = TreeNode::construct_tree_from_array(vec![
        Some(3),
        Some(0),
        Some(4),
        None,
        Some(2),
        None,
        None,
        Some(1),
    ]);
    let after_trim = TreeNode::trim_bst(root, 1, 3);
    let mut ret = vec![];
    TreeNode::in_order(after_trim, &mut ret);
    println!("{:?}", ret);
}
