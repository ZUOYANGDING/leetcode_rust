/**
 * Leetcode 106
 */
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug + From<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build_tree_from_in_and_post_order(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if inorder.len() < 1 {
            return None;
        }
        Self::dfs_for_in_and_post_order_rebuild(
            &inorder,
            &postorder,
            (0, inorder.len()),
            (0, postorder.len()),
        )
    }

    fn dfs_for_in_and_post_order_rebuild(
        inorder: &Vec<i32>,
        postorder: &Vec<i32>,
        inorder_range: (usize, usize),
        postorder_range: (usize, usize),
    ) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if postorder_range.0 == postorder_range.1 - 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(
                postorder[postorder_range.0],
            ))));
        }

        // use the last element of postorder range as the root to separate
        let root_num = postorder[postorder_range.1 - 1];
        let mut inorder_separation_point = inorder_range.0;
        while inorder_separation_point < inorder_range.1 {
            if inorder[inorder_separation_point] == root_num {
                break;
            } else {
                inorder_separation_point += 1;
            }
        }
        // get the range of left of new inorder range [a, b)
        let inorder_range_left_left = inorder_range.0;
        let inorder_range_left_right = inorder_separation_point;

        // println!("inorder left ({}, {})", inorder_range_left_left, inorder_range_left_right);

        // get the range of right of the new inorder range [a, b)
        let inorder_range_right_left =
            std::cmp::min(inorder_separation_point + 1, inorder_range.1 - 1);
        let inorder_range_right_right = inorder_range.1;

        // println!("inorder right ({}, {})", inorder_range_right_left, inorder_range_right_right);

        // get the length of left of new inorder range
        let new_left_len = inorder_range_left_right - inorder_range_left_left;

        // no need to get the length of the right of new inorder range
        // let new_right_len = inorder_range_left_right - inorder_range_right_left;

        // get the range of the left of the new postorder [a, b)
        let postorder_left_left = postorder_range.0;
        let postorder_left_right = postorder_left_left + new_left_len;

        // println!("postorder left ({}, {})", postorder_left_left, postorder_left_right);

        // get the range of the right of the new postorder[a, b)
        let postorder_right_left = std::cmp::min(postorder_left_right, postorder_range.1 - 1);
        let postorder_right_right = postorder_range.1 - 1;

        // println!("postorder right ({}, {})", postorder_right_left, postorder_right_right);

        // current node
        let node = Rc::new(RefCell::new(TreeNode::new(root_num)));

        // since apply [a, b) to seperate, b must larger than a, otherwise the child is None
        if inorder_range_left_left < inorder_range_left_right
            && postorder_left_left < postorder_left_right
        {
            node.borrow_mut().left = Self::dfs_for_in_and_post_order_rebuild(
                inorder,
                postorder,
                (inorder_range_left_left, inorder_range_left_right),
                (postorder_left_left, postorder_left_right),
            );
        }
        if inorder_range_right_left < inorder_range_right_right
            && postorder_right_left < postorder_right_right
        {
            node.borrow_mut().right = Self::dfs_for_in_and_post_order_rebuild(
                inorder,
                postorder,
                (inorder_range_right_left, inorder_range_right_right),
                (postorder_right_left, postorder_right_right),
            );
        }
        return Some(node);
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
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let mut ret = vec![];
    let tree_node = TreeNode::<i32>::build_tree_from_in_and_post_order(inorder.clone(), postorder);
    TreeNode::inorder_traverse(tree_node, &mut ret);
    assert_eq!(ret, inorder);
}
