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

    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let cur_num = root.clone().unwrap().borrow().val.to_owned();
        if cur_num == val {
            return root;
        } else if cur_num > val {
            return Self::search_bst(root.clone().unwrap().borrow().left.clone(), val);
        } else {
            return Self::search_bst(root.clone().unwrap().borrow().right.clone(), val);
        }
    }

    fn build_tree() -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(4)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(3)));
        node1.borrow_mut().left = Some(node3);
        node1.borrow_mut().right = Some(node4);
        root.borrow_mut().left = Some(node1);
        root.borrow_mut().right = Some(node2);
        Some(root)
    }

    fn bfs(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_front() {
                        ret.push(node.borrow().val);
                        if let Some(left_node) = &node.borrow().left {
                            queue.push_back(left_node.clone());
                        }
                        if let Some(right_node) = &node.borrow().right {
                            queue.push_back(right_node.clone());
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let tree = TreeNode::build_tree();
    let result_node = TreeNode::search_bst(tree, 2);
    let mut ret = vec![];
    TreeNode::bfs(result_node, &mut ret);
    println!("{:?}", ret);
}
