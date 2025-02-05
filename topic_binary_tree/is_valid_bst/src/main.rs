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

    /// This is not clean logic
    /// Use pre-order traverse, to compare the root and subtree/subnode,
    /// It is better to collect the whole tree by in-order, than check the vec is monotone increase or not
    pub fn is_valid_bst_not_clean_logic(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root, i64::MIN, i64::MAX)
    }

    /// Use in-order traverse to collect all nodes, than check the monotone increase or not
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result = vec![];
        Self::in_order(root, &mut result);
        Self::check(&result)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if node.is_none() {
            return true;
        }
        let cur_val = node.clone().unwrap().borrow().val as i64;
        let left_val = {
            if let Some(left_node) = &node.clone().unwrap().borrow().left {
                left_node.borrow().val as i64
            } else {
                i64::MIN
            }
        };
        let right_val = {
            if let Some(right_node) = &node.clone().unwrap().borrow().right {
                right_node.borrow().val as i64
            } else {
                i64::MAX
            }
        };
        if cur_val <= left_val || cur_val <= min {
            println!("{}, {}, {}", cur_val, left_val, min);
            return false;
        }
        if cur_val >= right_val || cur_val >= max {
            println!("{}, {}, {}", cur_val, right_val, max);
            return false;
        }
        let left_ret = Self::dfs(node.clone().unwrap().borrow().left.clone(), min, cur_val);
        let right_ret = Self::dfs(node.clone().unwrap().borrow().right.clone(), cur_val, max);
        return left_ret && right_ret;
    }

    fn in_order(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        Self::in_order(root.clone().unwrap().borrow().left.clone(), ret);
        ret.push(root.clone().unwrap().borrow().val);
        Self::in_order(root.clone().unwrap().borrow().right.clone(), ret);
    }

    fn check(result: &Vec<i32>) -> bool {
        let mut idx = 0;
        while idx < result.len() - 1 {
            if result[idx] >= result[idx + 1] {
                return false;
            }
            idx += 1;
        }
        true
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
}
fn main() {
    let nums_1 = vec![Some(2), Some(1), Some(3)];
    let root = TreeNode::construct_tree_from_array(nums_1);
    assert_eq!(TreeNode::is_valid_bst(root), true);
    let nums_2 = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
    let root = TreeNode::construct_tree_from_array(nums_2);
    assert_eq!(TreeNode::is_valid_bst(root), false);
    let nums_3 = vec![Some(5), Some(4), Some(6), None, None, Some(3), Some(7)];
    let root = TreeNode::construct_tree_from_array(nums_3);
    assert_eq!(TreeNode::is_valid_bst(root), false);
    let nums_4 = vec![
        Some(3),
        Some(1),
        Some(5),
        Some(0),
        Some(2),
        Some(4),
        Some(6),
        None,
        None,
        None,
        Some(3),
    ];
    let root = TreeNode::construct_tree_from_array(nums_4);
    assert_eq!(TreeNode::is_valid_bst(root), false);
    let nums_5 = vec![Some(2147483647)];
    let root = TreeNode::construct_tree_from_array(nums_5);
    assert_eq!(TreeNode::is_valid_bst(root), true);
}
