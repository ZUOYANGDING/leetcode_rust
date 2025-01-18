use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Debug + Into<i32>,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
