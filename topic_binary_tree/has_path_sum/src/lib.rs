/**
 * Leetcode 112
 */
use std::{cell::RefCell, fmt::Debug, ops::Add, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Debug + Clone + Into<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode<T>>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            Self::dfs(node.clone(), target_sum, 0)
        } else {
            false
        }
    }

    fn dfs(node: Rc<RefCell<TreeNode<T>>>, target_sum: i32, cur_sum: i32) -> bool {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            let cur_num = node.borrow().val.to_owned().into();
            if cur_sum + cur_num == target_sum {
                return true;
            } else {
                return false;
            }
        }

        let cur_num = node.borrow().val.to_owned().into();
        let left_ret = {
            if let Some(left_child) = &node.borrow().left {
                Self::dfs(left_child.clone(), target_sum, cur_num + cur_sum)
            } else {
                false
            }
        };
        let right_ret = {
            if let Some(right_child) = &node.borrow().right {
                Self::dfs(right_child.clone(), target_sum, cur_num + cur_sum)
            } else {
                false
            }
        };
        return left_ret || right_ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_binary_tree() -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let mut node_1 = TreeNode::new(1);
        let mut node_2 = TreeNode::new(2);
        let mut node_3 = TreeNode::new(3);
        let node_4 = TreeNode::new(4);
        let node_5 = TreeNode::new(5);
        node_2.left = Some(Rc::new(RefCell::new(node_4)));
        node_2.right = Some(Rc::new(RefCell::new(node_5)));
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        let root = Some(Rc::new(RefCell::new(node_1)));
        root
    }

    #[test]
    fn it_works() {
        assert!(TreeNode::has_path_sum(construct_binary_tree(), 7));
    }
}
