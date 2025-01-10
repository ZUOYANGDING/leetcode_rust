use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

/**
 * Leetcode 101
 */

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug + Eq + PartialEq,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    /// BFS
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let mut cur_layer = vec![];
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    if let Some(non_null_node) = node {
                        cur_layer.push(Some(non_null_node.borrow().val.to_owned()));

                        queue.push_back(non_null_node.borrow().left.clone());

                        queue.push_back(non_null_node.borrow().right.clone());
                    } else {
                        cur_layer.push(None);
                    }
                }
            }
            if !Self::check(&cur_layer) {
                return false;
            }
        }
        return true;
    }

    /// DFS
    pub fn is_symmetric_alter(root: Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
        if root.is_some() {
            return Self::compare(
                root.clone().unwrap().borrow().left.clone(),
                root.clone().unwrap().borrow().right.clone(),
            );
        }
        true
    }

    fn compare(
        left: Option<Rc<RefCell<TreeNode<T>>>>,
        right: Option<Rc<RefCell<TreeNode<T>>>>,
    ) -> bool {
        if left.is_some() && right.is_none() {
            return false;
        }
        if left.is_none() && right.is_some() {
            return false;
        }
        if left.is_none() && right.is_none() {
            return true;
        }
        let left_val = left.clone().unwrap().borrow().val.to_owned();
        let right_val = right.clone().unwrap().borrow().val.to_owned();
        if left_val != right_val {
            return false;
        }
        let left_of_left = left.clone().unwrap().borrow().left.clone();
        let right_of_left = left.clone().unwrap().borrow().right.clone();
        let right_of_right = right.clone().unwrap().borrow().right.clone();
        let left_of_right = right.clone().unwrap().borrow().left.clone();
        let outer_result = Self::compare(left_of_left, right_of_right);
        let inner_result = Self::compare(right_of_left, left_of_right);
        return outer_result && inner_result;
    }

    fn check(layer: &Vec<Option<T>>) -> bool {
        let mut head = 0;
        let mut end = layer.len() - 1;
        while head < end {
            if layer[head] != layer[end] {
                return false;
            }
            head += 1;
            end -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_binary_tree() -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let mut node_1 = TreeNode::new(1);
        let mut node_2 = TreeNode::new(2);
        let mut node_3 = TreeNode::new(2);
        let node_4 = TreeNode::new(3);
        let node_5 = TreeNode::new(4);
        let node_6 = TreeNode::new(4);
        let node_7 = TreeNode::new(3);
        node_2.left = Some(Rc::new(RefCell::new(node_4)));
        node_2.right = Some(Rc::new(RefCell::new(node_5)));
        node_3.left = Some(Rc::new(RefCell::new(node_6)));
        node_3.right = Some(Rc::new(RefCell::new(node_7)));
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        let root = Some(Rc::new(RefCell::new(node_1)));
        root
    }

    #[test]
    fn it_works() {
        let root = construct_binary_tree();
        assert_eq!(TreeNode::is_symmetric(root.clone()), true);
        assert_eq!(TreeNode::is_symmetric_alter(root), true);
    }
}
