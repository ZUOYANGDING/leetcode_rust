/**
 * Leetcode 530
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

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = vec![];
        Self::dfs(root, &mut values);
        Self::get_min_diff(&values)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }
        let cur_node = node.as_ref().unwrap();
        Self::dfs(cur_node.borrow().left.clone(), values);
        values.push(cur_node.borrow().val);
        Self::dfs(cur_node.borrow().right.clone(), values);
    }

    fn get_min_diff(values: &Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for idx in 0..values.len() - 1 {
            min = std::cmp::min(values[idx + 1] - values[idx], min);
        }
        min
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
}
fn main() {
    let root = TreeNode::construct_tree_from_array(vec![
        Some(1),
        Some(0),
        Some(48),
        None,
        None,
        Some(12),
        Some(49),
    ]);
    assert_eq!(TreeNode::get_minimum_difference(root), 1)
}
