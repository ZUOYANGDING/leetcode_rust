/**
 * Leetcode 404
 */
use std::{cell::RefCell, fmt::Debug, rc::Rc};

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

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        return Self::dfs(root.unwrap(), false);
    }

    fn dfs(node: Rc<RefCell<TreeNode<T>>>, is_left: bool) -> i32 {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            if is_left {
                return node.borrow().val.to_owned().into();
            } else {
                return 0_i32;
            }
        }

        let left = {
            if let Some(left_node) = &node.borrow().left {
                Self::dfs(left_node.clone(), true)
            } else {
                0
            }
        };
        let right = {
            if let Some(right_node) = &node.borrow().right {
                Self::dfs(right_node.clone(), false)
            } else {
                0
            }
        };
        return left + right;
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
        println!(
            "{:?}",
            TreeNode::sum_of_left_leaves(construct_binary_tree())
        );
    }
}
