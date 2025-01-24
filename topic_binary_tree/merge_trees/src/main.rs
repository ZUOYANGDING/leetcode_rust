/**
 * Leetcode 617
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
    T: Clone + Debug + Into<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode<i32>>>>,
        root2: Option<Rc<RefCell<TreeNode<i32>>>>,
    ) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if root1.is_none() && root2.is_none() {
            return None;
        }
        if root1.is_some() && root2.is_none() {
            return root1;
        }
        if root1.is_none() && root2.is_some() {
            return root2;
        }
        let cur_root1: i32 = root1.clone().unwrap().borrow().val.to_owned().into();
        let cur_root2: i32 = root2.clone().unwrap().borrow().val.to_owned().into();
        let cur_node = Rc::new(RefCell::new(TreeNode::new(cur_root1 + cur_root2)));
        cur_node.borrow_mut().left = Self::merge_trees(
            root1.clone().unwrap().borrow().left.clone(),
            root2.clone().unwrap().borrow().left.clone(),
        );
        cur_node.borrow_mut().right = Self::merge_trees(
            root1.clone().unwrap().borrow().right.clone(),
            root2.clone().unwrap().borrow().right.clone(),
        );
        Some(cur_node)
    }

    fn construct_binary_tree_one() -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(5)));
        node1.borrow_mut().left = Some(node3);
        root.borrow_mut().left = Some(node1);
        root.borrow_mut().right = Some(node2);
        Some(root)
    }

    fn construct_binary_tree_two() -> Option<Rc<RefCell<TreeNode<i32>>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(7)));
        node1.borrow_mut().right = Some(node3);
        node2.borrow_mut().right = Some(node4);
        root.borrow_mut().left = Some(node1);
        root.borrow_mut().right = Some(node2);
        Some(root)
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
    let tree_one = TreeNode::<i32>::construct_binary_tree_one();
    let tree_two = TreeNode::<i32>::construct_binary_tree_two();
    let after_merge = TreeNode::<i32>::merge_trees(tree_one, tree_two);
    let mut ret = vec![];
    TreeNode::inorder_traverse(after_merge, &mut ret);
    println!("{:?}", ret);
}
