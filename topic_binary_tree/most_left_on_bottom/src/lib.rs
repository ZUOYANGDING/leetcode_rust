use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

/**
 * Leetcode 513
 */
#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug + Into<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    /// BFS very slow
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            let mut ret = 0;
            while !queue.is_empty() {
                let mut cur_layer = vec![];
                for _ in 0..queue.len() {
                    if let Some(child) = queue.pop_front() {
                        cur_layer.push(child.borrow().val.to_owned());
                        if let Some(left_child) = &child.borrow().left {
                            queue.push_back(left_child.clone());
                        }
                        if let Some(right_child) = &child.borrow().right {
                            queue.push_back(right_child.clone());
                        }
                    }
                }
                ret = cur_layer[0].to_owned().into();
            }
            return ret;
        } else {
            return 0;
        }
    }

    /// DFS faster
    pub fn find_bottom_left_value_alter(root: Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        let mut most_left = 0_i32;
        let mut max_depth = 0_usize;
        if let Some(node) = root {
            Self::dfs(node.clone(), 1, &mut most_left, &mut max_depth);
        }
        return most_left;
    }

    fn dfs(
        node: Rc<RefCell<TreeNode<T>>>,
        cur_depth: usize,
        most_left: &mut i32,
        max_depth: &mut usize,
    ) {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            if cur_depth > *max_depth {
                *max_depth = cur_depth;
                *most_left = node.borrow().val.to_owned().into();
            }
            return;
        }
        if let Some(left_child) = &node.borrow().left {
            Self::dfs(left_child.clone(), cur_depth + 1, most_left, max_depth);
        }
        if let Some(right_child) = &node.borrow().right {
            Self::dfs(right_child.clone(), cur_depth + 1, most_left, max_depth);
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
        node_2.left = Some(Rc::new(RefCell::new(node_4)));
        node_2.right = Some(Rc::new(RefCell::new(node_5)));
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        let root = Some(Rc::new(RefCell::new(node_1)));
        root
    }

    #[test]
    fn it_works() {
        assert_eq!(
            TreeNode::find_bottom_left_value(construct_binary_tree()),
            TreeNode::find_bottom_left_value_alter(construct_binary_tree())
        );
    }
}
