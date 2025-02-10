/**
 * Leetcode 236
 */
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
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

    /// Since there is no duplicate number in the tree, check the number directly to find the p and q
    /// Apply the preorder DFS (left, right, mid) on tree recursively, which means start from the lowest level of the tree
    /// For each node, find the p and q in subtree with the current node as root
    /// return the first found node which is the ancestor of p and q
    /// the worst case will be O(n^2)
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        Self::preorder_dfs(root, (p_val, q_val))
    }

    fn preorder_dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        target: (i32, i32),
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() {
            return None;
        }
        let left_ret = Self::preorder_dfs(node.as_ref().unwrap().borrow().left.clone(), target);
        let right_ret = Self::preorder_dfs(node.as_ref().unwrap().borrow().right.clone(), target);
        if left_ret.is_some() {
            return left_ret;
        } else if right_ret.is_some() {
            return right_ret;
        } else {
            let mut res_p = false;
            let mut res_q = false;
            Self::dfs_check(node.clone(), target, &mut res_p, &mut res_q);
            if res_p && res_q {
                return node.clone();
            } else {
                return None;
            }
        }
    }

    fn dfs_check(
        node: Option<Rc<RefCell<TreeNode>>>,
        target: (i32, i32),
        res_p: &mut bool,
        res_q: &mut bool,
    ) {
        if node.is_none() {
            return;
        }
        let cur_val = node.as_ref().unwrap().borrow().val;
        if cur_val == target.0 {
            *res_p = true;
        }
        if cur_val == target.1 {
            *res_q = true;
        }
        if *res_p && *res_q {
            return;
        }
        Self::dfs_check(
            node.as_ref().unwrap().borrow().left.clone(),
            target,
            res_p,
            res_q,
        );
        Self::dfs_check(
            node.as_ref().unwrap().borrow().right.clone(),
            target,
            res_p,
            res_q,
        );
    }

    /// No need to apply the check on every node seperately
    pub fn lowest_common_ancestor_not_repeat_check(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        Self::preorder_dfs_alter(root, (p_val, q_val))
    }

    fn preorder_dfs_alter(
        node: Option<Rc<RefCell<TreeNode>>>,
        target: (i32, i32),
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none()
            || node.as_ref().unwrap().borrow().val == target.0
            || node.as_ref().unwrap().borrow().val == target.1
        {
            return node;
        }
        let left_ret =
            Self::preorder_dfs_alter(node.as_ref().unwrap().borrow().left.clone(), target);
        let right_ret =
            Self::preorder_dfs_alter(node.as_ref().unwrap().borrow().right.clone(), target);
        if left_ret.is_some() && right_ret.is_some() {
            return node;
        } else if left_ret.is_some() && right_ret.is_none() {
            return left_ret;
        } else if left_ret.is_none() && right_ret.is_some() {
            return right_ret;
        } else {
            None
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
            if idx < array.len() {
                if array[idx].is_some() {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(array[idx].unwrap())));
                    cur_node.as_ref().unwrap().borrow_mut().left = Some(left_child.clone());
                    queue.push_back(Some(left_child));
                }
                idx += 1;
            }
            if idx < array.len() {
                if array[idx].is_some() {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(array[idx].unwrap())));
                    cur_node.as_ref().unwrap().borrow_mut().right = Some(right_child.clone());
                    queue.push_back(Some(right_child));
                }
                idx += 1;
            }
        }
        Some(root)
    }
}
fn main() {
    let root = TreeNode::construct_tree_from_array(&vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    let ret_1 = TreeNode::lowest_common_ancestor(
        root.clone(),
        Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    );
    let ret_2 = TreeNode::lowest_common_ancestor_not_repeat_check(
        root.clone(),
        Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    );
    assert_eq!(ret_1.unwrap().borrow().val, 3);
    assert_eq!(ret_2.unwrap().borrow().val, 3);
}
