/**
 * Recursive with stack (OS stack) is a strategy matching DFS idea naturelly
 * Iteration with queue is a stragegy match BFS idea natruelly
 */
use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

/// normal structrue for binary tree
#[derive(Debug, PartialEq, Eq)]
struct BinaryTreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
    right: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
}

impl<T> BinaryTreeNode<T>
where
    T: Clone + Debug + Into<f64> + Into<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn bfs(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>, ret: &mut Vec<Vec<T>>) {
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let mut cur_layer = Vec::new();
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                    cur_layer.push(node.borrow().val.to_owned());
                }
            }
            ret.push(cur_layer);
        }
    }

    // leetcode 102
    pub fn level_order(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Vec<Vec<T>> {
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        let mut ret = vec![];
        while !queue.is_empty() {
            let mut cur_layer = vec![];
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                    cur_layer.push(node.borrow().val.to_owned())
                }
            }
            ret.push(cur_layer);
        }
        ret
    }

    // leetcode 107
    pub fn level_order_bottom(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Vec<Vec<T>> {
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        let mut ret = vec![];
        while !queue.is_empty() {
            let mut cur_layer = vec![];
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                    cur_layer.push(node.borrow().val.to_owned())
                }
            }
            ret.push(cur_layer);
        }
        ret.reverse();
        ret
    }

    // leetcode 199
    pub fn right_side_view(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Vec<T> {
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        let mut ret = vec![];
        while !queue.is_empty() {
            let layer_length = queue.len();
            for idx in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                    if idx == layer_length - 1 {
                        ret.push(node.borrow().val.to_owned());
                    }
                }
            }
        }
        ret
    }

    // leetcode 637
    pub fn average_of_levels(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Vec<f64> {
        let mut queue = VecDeque::new();
        if root.is_none() {
            return vec![0_f64];
        }
        queue.push_back(root);
        let mut ret = vec![];
        while !queue.is_empty() {
            let mut cur_layer_sum = 0_f64;
            let cur_layer_length: f64 = queue.len() as f64;
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                    let value: f64 = node.borrow().val.to_owned().into();
                    cur_layer_sum += value;
                }
            }
            ret.push(cur_layer_sum / cur_layer_length);
        }
        ret
    }

    // leetcode 515
    pub fn largest_values(root: Option<Rc<RefCell<BinaryTreeNode<T>>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut ret = vec![];
        if let Some(root) = root {
            queue.push_back(root);
            while !queue.is_empty() {
                let mut cur_max = i32::MIN;
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_front() {
                        let val = node.borrow().val.to_owned().into();
                        if cur_max < val {
                            cur_max = val;
                        }
                        if let Some(left_node) = &node.borrow().left {
                            queue.push_back(left_node.clone());
                        }
                        if let Some(right_node) = &node.borrow().right {
                            queue.push_back(right_node.clone());
                        }
                    }
                }
                ret.push(cur_max);
            }
        }
        ret
    }
}

/// Structure for muti-children tree(heap/graph)
#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    children: Option<Rc<RefCell<Vec<TreeNode<T>>>>>,
}

struct Tree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug,
{
    fn new(val: T) -> Self {
        Self {
            val,
            children: None,
        }
    }

    fn add_child(&mut self, child: TreeNode<T>) {
        if self.children.is_none() {
            self.children = Some(Rc::new(RefCell::new(Vec::new())));
        }
        if let Some(children) = &self.children {
            children.borrow_mut().push(child);
        }
    }
}

impl<T> Tree<T>
where
    T: Clone + Debug,
{
    fn new(val: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(TreeNode::new(val)))),
        }
    }

    // leetcode 429
    fn bfs(&self, ret: &mut Vec<Vec<T>>) {
        if let Some(root_node) = &self.root {
            let mut queue = VecDeque::new();
            queue.push_back(root_node.clone());
            while !queue.is_empty() {
                let mut cur_layer = vec![];
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_front() {
                        cur_layer.push(node.borrow().val.to_owned());
                        if let Some(children) = &node.borrow().children {
                            for child in children.borrow().iter() {
                                queue.push_back(Rc::new(RefCell::new(child.clone())));
                            }
                        }
                    }
                }
                ret.push(cur_layer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_binary_tree() -> Option<Rc<RefCell<BinaryTreeNode<i32>>>> {
        let mut node_1 = BinaryTreeNode::new(1);
        let mut node_2 = BinaryTreeNode::new(2);
        let mut node_3 = BinaryTreeNode::new(3);
        let node_4 = BinaryTreeNode::new(4);
        let node_5 = BinaryTreeNode::new(5);
        let node_6 = BinaryTreeNode::new(6);
        node_2.left = Some(Rc::new(RefCell::new(node_4)));
        node_2.right = Some(Rc::new(RefCell::new(node_5)));
        node_3.right = Some(Rc::new(RefCell::new(node_6)));
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        let root = Some(Rc::new(RefCell::new(node_1)));
        root
    }

    #[test]
    fn it_works() {
        let root = construct_binary_tree();
        let mut ret = vec![];
        BinaryTreeNode::bfs(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
        println!("{:?}", BinaryTreeNode::right_side_view(root.clone()));

        println!("{:?}", BinaryTreeNode::largest_values(root.clone()))
    }

    //              1
    //          2   3   4
    //      5   6   7   8   9
    //  10  11      12          13
    #[test]
    fn test_multichildren_tree() {
        let mut tree = Tree::new(1);
        if let Some(root) = &mut tree.root {
            for i in 2..5 {
                root.borrow_mut().add_child(TreeNode::new(i));
            }
            if let Some(children) = &root.borrow_mut().children {
                if let Some(node_2) = children.borrow_mut().get_mut(0) {
                    for i in 5..7 {
                        node_2.add_child(TreeNode::new(i));
                    }
                    if let Some(children) = &node_2.children {
                        if let Some(node_5) = children.borrow_mut().get_mut(0) {
                            for i in 10..12 {
                                node_5.add_child(TreeNode::new(i));
                            }
                        }
                    }
                }

                if let Some(node_4) = children.borrow_mut().get_mut(2) {
                    for i in 7..10 {
                        node_4.add_child(TreeNode::new(i));
                    }
                    if let Some(children) = &node_4.children {
                        if let Some(node_7) = children.borrow_mut().get_mut(0) {
                            node_7.add_child(TreeNode::new(12));
                        }
                        if let Some(node_9) = children.borrow_mut().get_mut(2) {
                            node_9.add_child(TreeNode::new(13));
                        }
                    }
                }
            }
        }

        let mut v = vec![];
        tree.bfs(&mut v);
        println!("{:?}", v);
        v.clear();
    }
}
