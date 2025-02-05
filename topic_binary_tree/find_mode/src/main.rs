use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // Better than use HashMap or BTreeMap to counting
    // But still need to store the whole tree first
    pub fn find_mode_with_extra_space(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut all = vec![];
        Self::dfs_extra_space(root, &mut all);
        if all.len() == 1 {
            return all.clone();
        }
        let mut ret = vec![];
        Self::get_mode(&all, &mut ret);
        ret
    }

    fn dfs_extra_space(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }
        let cur_node = node.as_ref().unwrap();
        Self::dfs_extra_space(cur_node.borrow().left.clone(), ret);
        ret.push(cur_node.borrow().val);
        Self::dfs_extra_space(cur_node.borrow().right.clone(), ret);
    }

    fn get_mode(all: &Vec<i32>, ret: &mut Vec<i32>) {
        let mut slow_idx = 0;
        let mut fast_idx = 0;
        let mut max_length = 0;
        while fast_idx < all.len() {
            while fast_idx < all.len() && all[slow_idx] == all[fast_idx] {
                fast_idx += 1;
            }
            if fast_idx - slow_idx > max_length {
                max_length = fast_idx - slow_idx;
                ret.clear();
                ret.push(all[slow_idx]);
            } else if fast_idx - slow_idx == max_length {
                ret.push(all[slow_idx]);
            }
            slow_idx = fast_idx;
        }
    }

    // Basically same idea as the approach with extra space
    // Which is inorder dfs to go through the tree to have a increasing order of the array
    // But `get_mode` approach directly with the dfs traverse
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        Self::dfs(root, &mut ret, &mut i32::MIN, &mut 0, &mut 0);
        ret
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        ret: &mut Vec<i32>,
        prev_val: &mut i32,
        cur_count: &mut usize,
        max_count: &mut usize,
    ) {
        if node.is_none() {
            return;
        }
        let cur_node = node.as_ref().unwrap();
        let cur_val = cur_node.borrow().val;
        Self::dfs(
            cur_node.borrow().left.clone(),
            ret,
            prev_val,
            cur_count,
            max_count,
        );
        if *prev_val == i32::MIN {
            *cur_count = 1;
        } else if cur_val == *prev_val {
            *cur_count += 1;
        } else {
            *cur_count = 1;
        }
        if *cur_count > *max_count {
            ret.clear();
            ret.push(cur_val);
            *max_count = *cur_count;
        } else if *cur_count == *max_count {
            ret.push(cur_val);
        }
        *prev_val = cur_val;
        Self::dfs(
            cur_node.borrow().right.clone(),
            ret,
            prev_val,
            cur_count,
            max_count,
        );
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
    let root = TreeNode::construct_tree_from_array(vec![Some(1), None, Some(2), Some(2)]);
    println!("{:?}", TreeNode::find_mode(root));
}
