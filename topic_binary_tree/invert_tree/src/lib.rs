/**
 * Leetcode 226
 */
use std::{cell::RefCell, fmt::Debug, rc::Rc};

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

    fn invert_tree(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if let Some(node) = root {
            let left_child = Self::invert_tree(node.borrow_mut().left.clone());
            let right_child = Self::invert_tree(node.borrow_mut().right.clone());
            node.borrow_mut().left = right_child;
            node.borrow_mut().right = left_child;
            return Some(node);
        } else {
            root
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

    fn preorder_travers(root: Option<Rc<RefCell<TreeNode<i32>>>>, ret: &mut Vec<i32>) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                ret.push(node.borrow().val.to_owned());
                return;
            }
            ret.push(node.borrow().val.to_owned());
            preorder_travers(node.borrow().left.clone(), ret);
            preorder_travers(node.borrow().right.clone(), ret);
        }
    }

    #[test]
    fn it_works() {
        let root = construct_binary_tree();
        let mut ret = vec![];
        preorder_travers(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
        TreeNode::invert_tree(root.clone());
        preorder_travers(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
    }
}
