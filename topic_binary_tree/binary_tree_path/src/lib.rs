/**
 * Leetcode 257
 */
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug + Display,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<String> {
        if root.is_none() {
            return vec![];
        }
        let cur_path = "";
        let mut ret = vec![];
        Self::dfs(root.unwrap(), cur_path, &mut ret);
        ret
    }

    fn dfs(node: Rc<RefCell<TreeNode<T>>>, cur_path: &str, ret: &mut Vec<String>) {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            let val = node.borrow().val.to_owned();
            let path = format!("{}{}", cur_path, val);
            ret.push(path.to_string());
            return;
        }

        let cur_val = node.borrow().val.to_owned();
        let path = format!("{}{}->", cur_path, cur_val);
        if let Some(left_node) = &node.borrow().left {
            Self::dfs(left_node.clone(), path.as_str(), ret);
        }
        if let Some(right_node) = &node.borrow().right {
            Self::dfs(right_node.clone(), path.as_str(), ret);
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
        let root = construct_binary_tree();
        println!("{:?}", TreeNode::binary_tree_paths(root));
    }
}
