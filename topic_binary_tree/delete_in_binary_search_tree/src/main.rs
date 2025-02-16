use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // cannot find target node
        if root.is_none() {
            return root;
        }
        let cur_node = root.clone().unwrap();
        if cur_node.borrow().val == key {
            if cur_node.borrow().left.is_none() && cur_node.borrow().right.is_none() {
                // target node is leaf
                // just delete
                return None;
            } else if cur_node.borrow().left.is_none() && cur_node.borrow().right.is_some() {
                // target node left is none, use right subtree to replace the target
                return cur_node.borrow().right.clone();
            } else if cur_node.borrow().left.is_some() && cur_node.borrow().right.is_none() {
                // target node right is none, use left subtree to replace the target
                return cur_node.borrow().left.clone();
            } else {
                // both left and right not none
                // find the most left of right
                let mut tmp = cur_node.borrow().right.clone();
                while tmp.clone().unwrap().borrow().left.is_some() {
                    tmp = tmp.clone().unwrap().borrow().left.clone();
                }
                // set the current left child as left child of the most left of current right child
                tmp.as_ref().unwrap().borrow_mut().left = cur_node.borrow().left.clone();
                // replace current as current right
                return cur_node.borrow().right.clone();
            }
        } else if cur_node.borrow().val < key {
            let right_node = Self::delete_node(cur_node.borrow().right.clone(), key);
            cur_node.borrow_mut().right = right_node;
        } else {
            let left_node = Self::delete_node(cur_node.borrow().left.clone(), key);
            cur_node.borrow_mut().left = left_node;
        }
        drop(cur_node);
        root
    }

    fn construct_tree_from_array(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.is_empty() || array[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(array[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut idx = 1; // Start from index 1 since root is already used

        while !queue.is_empty() && idx < array.len() {
            let cur_node = queue.pop_front().unwrap();

            // Process left child
            if idx < array.len() {
                if let Some(val) = array[idx] {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    cur_node.borrow_mut().left = Some(left_child.clone());
                    queue.push_back(left_child);
                }
                idx += 1;
            }

            // Process right child
            if idx < array.len() {
                if let Some(val) = array[idx] {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    cur_node.borrow_mut().right = Some(right_child.clone());
                    queue.push_back(right_child);
                }
                idx += 1;
            }
        }
        Some(root)
    }

    fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let cur_node = root.as_ref().unwrap().clone();
        Self::in_order(cur_node.borrow().left.clone(), ret);
        ret.push(cur_node.borrow().val);
        Self::in_order(cur_node.borrow().right.clone(), ret);
    }
}

fn main() {
    let root = TreeNode::construct_tree_from_array(vec![
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        Some(7),
    ]);
    let after_delete = TreeNode::delete_node(root, 3);
    let mut ret = vec![];
    TreeNode::in_order(after_delete, &mut ret);
    println!("{:?}", ret);
}
