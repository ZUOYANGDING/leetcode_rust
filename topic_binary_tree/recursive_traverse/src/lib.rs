use std::{borrow::Borrow, cell::RefCell, fmt::Debug, io::SeekFrom, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Debug + Clone,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn inorder_traverse(&self, result: &mut Vec<T>) {
        if self.left.is_none() && self.right.is_none() {
            result.push(self.borrow().val.to_owned());
            return;
        }
        if let Some(left_child) = self.left.as_ref() {
            left_child.as_ref().borrow().inorder_traverse(result);
        }
        result.push(self.borrow().val.to_owned());
        if let Some(right_child) = self.right.as_ref() {
            right_child.as_ref().borrow().inorder_traverse(result);
        }
    }

    fn preorder_traverse(&self, result: &mut Vec<T>) {
        if self.left.is_none() && self.right.is_none() {
            result.push(self.borrow().val.to_owned());
            return;
        }
        result.push(self.borrow().val.to_owned());
        if let Some(left_child) = self.left.as_ref() {
            left_child.as_ref().borrow().preorder_traverse(result);
        }
        if let Some(right_child) = self.right.as_ref() {
            right_child.as_ref().borrow().preorder_traverse(result);
        }
    }

    fn postorder_traverse(&self, result: &mut Vec<T>) {
        if self.left.is_none() && self.right.is_none() {
            result.push(self.borrow().val.to_owned());
            return;
        }
        if let Some(left_child) = self.left.as_ref() {
            left_child.as_ref().borrow().postorder_traverse(result);
        }
        if let Some(right_child) = self.right.as_ref() {
            right_child.as_ref().borrow().postorder_traverse(result);
        }
        result.push(self.borrow().val.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut node_1 = TreeNode::new(1);
        let mut node_2 = TreeNode::new(2);
        let mut node_3 = TreeNode::new(3);
        let node_4 = TreeNode::new(4);
        let node_5 = TreeNode::new(5);
        let node_6 = TreeNode::new(6);
        node_2.left = Some(Rc::new(RefCell::new(node_4)));
        node_2.right = Some(Rc::new(RefCell::new(node_5)));
        node_3.right = Some(Rc::new(RefCell::new(node_6)));
        node_1.left = Some(Rc::new(RefCell::new(node_2)));
        node_1.right = Some(Rc::new(RefCell::new(node_3)));
        let mut ret = vec![];
        node_1.inorder_traverse(&mut ret);
        println!("{:?}", ret);
        ret.clear();
        node_1.preorder_traverse(&mut ret);
        println!("{:?}", ret);
        ret.clear();
        node_1.postorder_traverse(&mut ret);
        println!("{:?}", ret);
    }
}
