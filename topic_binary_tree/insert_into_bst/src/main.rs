/**
 * Leetcode 701
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

    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        Self::insert(root.clone().unwrap(), val);
        root
    }

    fn insert(node: Rc<RefCell<TreeNode>>, val: i32) {
        let cur_val = node.borrow().val;
        if val < cur_val {
            if node.borrow().left.is_none() {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                return;
            } else {
                Self::insert(node.borrow().left.clone().unwrap(), val);
            }
        } else {
            if node.borrow().right.is_none() {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                return;
            } else {
                Self::insert(node.borrow().right.clone().unwrap(), val);
            }
        }
    }

    fn construct_tree_from_array(array: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.len() == 0 || array[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(array[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Some(root.clone()));
        let mut idx = 1;
        while !queue.is_empty() && idx < array.len() {
            let cur_node = queue.pop_front().unwrap();
            if idx < array.len() {
                if array[idx].is_some() {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(array[idx].unwrap())));
                    cur_node.as_ref().unwrap().borrow_mut().left = Some(left_child.clone());
                    queue.push_back(Some(left_child));
                }
                idx += 1;
            }
            if idx < array.len() {
                if array[idx].is_some() {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(array[idx].unwrap())));
                    cur_node.as_ref().unwrap().borrow_mut().right = Some(right_child.clone());
                    queue.push_back(Some(right_child));
                }
                idx += 1;
            }
        }
        Some(root)
    }

    fn inorder_dfs(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        Self::inorder_dfs(root.as_ref().unwrap().borrow().left.clone(), ret);
        ret.push(root.as_ref().unwrap().borrow().val);
        Self::inorder_dfs(root.as_ref().unwrap().borrow().right.clone(), ret);
    }
}

fn main() {
    let root = TreeNode::construct_tree_from_array(&vec![
        Some(5),
        None,
        Some(14),
        Some(10),
        Some(77),
        None,
        None,
        None,
        Some(95),
        None,
        None,
    ]);
    let after_insert = TreeNode::insert_into_bst(root, 4);
    let mut inorder_ret = vec![];
    TreeNode::inorder_dfs(after_insert, &mut inorder_ret);
    assert_eq!(inorder_ret, vec![4, 5, 10, 14, 77, 95]);
}
