/**
 * Leetcode 235
 * It is binary search tree. The value of ancestor should in range [p, q]
 * Assumption: Go through the tree top down recursively, the first node whose value in [p, q] is the lowest common ancestor
 * Prove:
 *      Suppose there are 2 nodes, whose value are i and k. p < i < k < q
 *      And the common lowest ancestor is i, but first node met is k.
 *      Since we traverse top down which means the first node we met should be great than left and less than right,
 *      which means if we met k first k<i, which is contradict to our supposing.
 */
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

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        let (left, right) = {
            if p_val < q_val {
                (p_val, q_val)
            } else {
                (q_val, p_val)
            }
        };
        Self::dfs(root, left, right)
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        left: i32,
        right: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() {
            return None;
        }
        let cur_val = node.as_ref().unwrap().borrow().val;
        if cur_val > right {
            Self::dfs(node.as_ref().unwrap().borrow().left.clone(), left, right)
        } else if cur_val < left {
            Self::dfs(node.as_ref().unwrap().borrow().right.clone(), left, right)
        } else {
            node.clone()
        }
    }

    fn construct_tree_from_array(array: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.len() == 0 || array[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(array[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Some(root.clone()));
        let mut idx = 1;
        while !queue.is_empty() && idx < array.len() {
            let cur_node = queue.pop_front().unwrap();
            if idx < array.len() && array[idx].is_some() {
                let left_child = Rc::new(RefCell::new(TreeNode::new(array[idx].unwrap())));
                cur_node.as_ref().unwrap().borrow_mut().left = Some(left_child.clone());
                queue.push_back(Some(left_child));
                idx += 1;
            }
            if idx < array.len() && array[idx].is_some() {
                let right_child = Rc::new(RefCell::new(TreeNode::new(array[idx].unwrap())));
                cur_node.as_ref().unwrap().borrow_mut().right = Some(right_child.clone());
                queue.push_back(Some(right_child));
                idx += 1;
            }
        }
        Some(root)
    }
}

fn main() {
    let root = TreeNode::construct_tree_from_array(&vec![
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ]);
    let ret = TreeNode::lowest_common_ancestor(
        root,
        Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        Some(Rc::new(RefCell::new(TreeNode::new(8)))),
    );
    assert_eq!(ret.unwrap().borrow().val, 6);
}
