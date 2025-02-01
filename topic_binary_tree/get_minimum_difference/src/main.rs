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

    fn contruct_tree_from_array(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.len() < 1 || array[0].is_none() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(array[0].unwrap()))));
        if array.len() < 2 {
            return root;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut idx = 0;
        while !queue.is_empty() && idx < array.len() {
            for _ in 0..queue.len() {
                if let Some(cur_node) = queue.pop_front().unwrap() {
                    if idx < array.len() - 2 {
                        let cur_left = if let Some(val) = array[idx + 1] {
                            Some(Rc::new(RefCell::new(TreeNode::new(val))))
                        } else {
                            None
                        };
                        cur_node.borrow_mut().left = cur_left.clone();
                        queue.push_back(cur_left);
                        idx += 1;
                    }
                    if idx < array.len() - 1 {
                        let cur_right = if let Some(val) = array[idx + 1] {
                            Some(Rc::new(RefCell::new(TreeNode::new(val))))
                        } else {
                            None
                        };
                        cur_node.borrow_mut().right = cur_right.clone();
                        queue.push_back(cur_right);
                        idx += 1;
                    }
                }
            }
        }
        root
    }
}
fn main() {
    let root = TreeNode::contruct_tree_from_array(vec![
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
