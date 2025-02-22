/**
 * Leetcode 538
 */
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

    /// The idea is get sum of the whole tree, than go through the whole tree inorder,
    /// then set up the val of each node as sum, after setting, substract the current node val from sum
    pub fn convert_bst_stupid(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = Self::dfs_sum(root.clone());
        Self::dfs_change(root.clone(), &mut sum);
        root
    }

    fn dfs_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let cur_node = root.as_ref().unwrap().borrow();
        let left_sum = Self::dfs_sum(cur_node.left.clone());
        let right_sum = Self::dfs_sum(cur_node.right.clone());
        return cur_node.val + left_sum + right_sum;
    }

    fn dfs_change(node: Option<Rc<RefCell<TreeNode>>>, total_sum: &mut i32) {
        if node.is_none() {
            return;
        }
        let mut cur_node = node.as_ref().unwrap().borrow_mut();
        Self::dfs_change(cur_node.left.clone(), total_sum);
        let cur_val = cur_node.val;
        cur_node.val = *total_sum;
        *total_sum -= cur_val;
        Self::dfs_change(cur_node.right.clone(), total_sum);
    }

    /// Better way, right->mid->left, record the sum and set the node val as sum.
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::right_first_change(root.clone(), &mut sum);
        root
    }

    pub fn right_first_change(node: Option<Rc<RefCell<TreeNode>>>, cur_sum: &mut i32) {
        if node.is_none() {
            return;
        }
        let mut cur_node = node.as_ref().unwrap().borrow_mut();
        Self::right_first_change(cur_node.right.clone(), cur_sum);
        cur_node.val += *cur_sum;
        *cur_sum = cur_node.val;
        Self::right_first_change(cur_node.left.clone(), cur_sum);
    }

    pub fn construct_tree_from_array(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.is_empty() || array[0].is_none() {
            return None;
        }
        let mut queue = VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(array[0].unwrap())));
        queue.push_back(root.clone());
        let mut idx = 1;
        while !queue.is_empty() && idx < array.len() {
            let cur_node = queue.pop_front().unwrap();
            if idx < array.len() {
                if let Some(left_val) = array[idx] {
                    let node = Rc::new(RefCell::new(TreeNode::new(left_val)));
                    queue.push_back(node.clone());
                    cur_node.borrow_mut().left = Some(node);
                }
                idx += 1;
            }

            if idx < array.len() {
                if let Some(right_val) = array[idx] {
                    let node = Rc::new(RefCell::new(TreeNode::new(right_val)));
                    queue.push_back(node.clone());
                    cur_node.borrow_mut().right = Some(node);
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
        let cur_node = root.as_ref().unwrap().borrow();
        Self::in_order(cur_node.left.clone(), ret);
        ret.push(cur_node.val);
        Self::in_order(cur_node.right.clone(), ret);
    }
}

fn main() {
    let root = TreeNode::construct_tree_from_array(vec![
        Some(4),
        Some(1),
        Some(6),
        Some(0),
        Some(2),
        Some(5),
        Some(7),
        None,
        None,
        None,
        Some(3),
        None,
        None,
        None,
        Some(8),
    ]);

    // let after_convert_1 = TreeNode::convert_bst_stupid(root.clone());
    let after_convert_2 = TreeNode::convert_bst(root);
    let mut ret = vec![];
    TreeNode::in_order(after_convert_2, &mut ret);
    println!("{:?}", ret);
}
