use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Debug + Clone,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        Self::helper(root).0
    }

    fn helper(root: Option<Rc<RefCell<TreeNode<T>>>>) -> (bool, i32) {
        if root.is_none() {
            return (true, 0);
        }
        let (left_check, left_depth) = Self::helper(root.clone().unwrap().borrow().left.clone());
        let (right_check, right_depth) = Self::helper(root.clone().unwrap().borrow().right.clone());
        let current_depth = std::cmp::max(left_depth, right_depth);
        let current_check = (left_check && right_check) && ((left_depth - right_depth).abs() <= 1);
        return (current_check, current_depth + 1);
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
        assert!(TreeNode::is_balanced(construct_binary_tree()));
    }
}
