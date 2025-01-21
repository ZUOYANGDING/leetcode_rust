/**
 * Leetcode 654
 */
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug + From<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if nums.len() < 1 {
            None
        } else {
            Self::dfs(&nums, (0, nums.len()))
        }
    }

    fn dfs(nums: &Vec<i32>, range: (usize, usize)) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if range.0 == range.1 - 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[range.0].into()))));
        }

        // get max idx
        let max_idx = Self::get_max_idx(nums, range.0, range.1);
        let root = Rc::new(RefCell::new(TreeNode::new(nums[max_idx].into())));
        if range.0 < max_idx {
            root.borrow_mut().left = Self::dfs(nums, (range.0, max_idx));
        }
        if max_idx + 1 < range.1 {
            root.borrow_mut().right = Self::dfs(nums, (max_idx + 1, range.1));
        }
        Some(root)
    }

    fn get_max_idx(nums: &Vec<i32>, head: usize, tail: usize) -> usize {
        let mut max_idx = head;
        let mut max = 0;
        for idx in head..tail {
            if nums[idx] >= max {
                max_idx = idx;
                max = nums[idx];
            }
        }
        max_idx
    }

    fn inorder_traverse(node: Option<Rc<RefCell<TreeNode<T>>>>, result: &mut Vec<T>) {
        if node.is_none() {
            return;
        }
        if node.clone().unwrap().borrow().left.is_none()
            && node.clone().unwrap().borrow().right.is_none()
        {
            result.push(node.unwrap().borrow().val.to_owned());
            return;
        }
        Self::inorder_traverse(node.clone().unwrap().borrow().left.clone(), result);
        result.push(node.clone().unwrap().borrow().val.to_owned());
        Self::inorder_traverse(node.clone().unwrap().borrow().right.clone(), result);
    }
}
fn main() {
    let nums = vec![3, 2, 1, 6, 0, 5];
    let tree_node = TreeNode::construct_maximum_binary_tree(nums);
    let mut ret: Vec<i32> = vec![];
    TreeNode::inorder_traverse(tree_node, &mut ret);
    println!("{:?}", ret);
}
