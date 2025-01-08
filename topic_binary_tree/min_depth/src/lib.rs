/**
 * leetcode 111
 */
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

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

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        if let Some(node) = root {
            let left_depth = Self::min_depth(node.borrow().left.clone());
            let right_depth = Self::min_depth(node.borrow().right.clone());
            match (left_depth, right_depth) {
                (0, d) | (d, 0) => d + 1,
                (ld, rd) => ld.min(rd) + 1,
            }
        } else {
            0
        }
    }

    pub fn min_depth_by_bfs(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
            let mut layers = 0;
            while !queue.is_empty() {
                layers += 1;
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_front() {
                        if node.borrow().left.is_none() && node.borrow().right.is_none() {
                            return layers;
                        }
                        if let Some(left_child) = &node.borrow().left {
                            queue.push_back(left_child.clone());
                        }
                        if let Some(right_child) = &node.borrow().right {
                            queue.push_back(right_child.clone());
                        }
                    }
                }
            }
            return layers;
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
        assert_eq!(
            TreeNode::min_depth(root.clone()),
            TreeNode::min_depth_by_bfs(root.clone())
        )
    }
}
