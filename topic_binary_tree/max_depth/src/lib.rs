/**
 * leetcode 104
 *
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
    T: Clone + Debug,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return 1;
            }
            let left_depth = Self::max_depth(node.borrow().left.clone());
            let right_depth = Self::max_depth(node.borrow().right.clone());
            let ret = {
                if left_depth >= right_depth {
                    left_depth + 1
                } else {
                    right_depth + 1
                }
            };
            ret
        } else {
            0
        }
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
        let node_6 = TreeNode::new(6);
        node_2.left = Some(Rc::new(RefCell::new(node_4)));
        node_2.right = Some(Rc::new(RefCell::new(node_5)));
        node_3.right = Some(Rc::new(RefCell::new(node_6)));
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        let root = Some(Rc::new(RefCell::new(node_1)));
        root
    }

    #[test]
    fn it_works() {
        let root = construct_binary_tree();
        assert_eq!(TreeNode::max_depth(root), 3);
    }
}
