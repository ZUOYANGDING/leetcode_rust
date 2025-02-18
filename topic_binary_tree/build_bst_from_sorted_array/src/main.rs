/**
 * Leetcode 108
 */
use std::{cell::RefCell, rc::Rc, vec};

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

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::divide_and_build(&nums, 0_i32, (nums.len() - 1) as i32)
    }

    fn divide_and_build(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
        root.borrow_mut().left = Self::divide_and_build(nums, left, mid - 1);
        root.borrow_mut().right = Self::divide_and_build(nums, mid + 1, right);
        Some(root)
    }
    fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let cur_node = root.as_ref().unwrap().borrow();
        ret.push(cur_node.val);
        Self::pre_order(cur_node.left.clone(), ret);
        Self::pre_order(cur_node.right.clone(), ret);
    }
}
fn main() {
    let root = TreeNode::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
    let mut ret = vec![];
    TreeNode::pre_order(root, &mut ret);
    println!("{:?}", ret);
}
