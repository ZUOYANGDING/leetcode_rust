/**
 * The iteration traverse using a stack to simulate the recursive process
 * The difference is, recursive way does not need to consider store visiting status,
 * that if children of current node has already been visited or not (pushed into stack in expected order).
 * Because in recursion, the children visited or not is marked by the recursion for the child itself.
 * When recursion step back to current node, all children have been visited already
 *
 * This can be found in [inorder_traverse] easily. After DFS to the most left node,
 * the first pop() will provide the most left leave node, just store the value and continue the pop()
 * But then the mid node will be pop(), the dealing process become store the value and iteration its right subtree
 * Therefore, the problem is when a node pop from a stack, we have no idea to know if we need to iterate its right subtree or not
 * that means we need to put the exactly same logic (code) again when need to visit the right subtree
 * However, in recursion way, everything (code and visiting status) will be remebered by stack in system stack
 *
 * To solve the problem, we can use a marker to mark if the node's children has been visited or not
 * All functions name with "*_alter" apply this strategy
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
    T: Clone + Debug,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    /// use stack to control the node visiting order
    /// use a pointer to indicate the node current visited
    fn inorder_traverse(root: Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        let mut stack = Vec::new();
        let mut cur_node = root.clone();
        while !stack.is_empty() || cur_node.is_some() {
            // DFS to left most leaf, and push all nodes on path
            while let Some(node) = cur_node {
                stack.push(node.clone());
                cur_node = node.borrow().left.clone();
            }
            // visit each node in stack, also load the right child
            if let Some(node) = stack.pop() {
                res.push(node.borrow().val.to_owned());
                cur_node = node.borrow().right.clone();
            }
        }
    }

    /// push right before left into stack, beacuse need to visit left before right
    /// deal with mid first and always visited mid first
    /// no need extra pointer
    fn preorder_traverse(root: Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        let mut stack = Vec::new();
        stack.push(root.clone());
        while let Some(node) = stack.pop() {
            if let Some(n) = node {
                res.push(n.borrow().val.clone());
                stack.push(n.borrow().right.clone());
                stack.push(n.borrow().left.clone());
            }
        }
    }

    /// push left before right into stack, then same logic as preorder_traverse, the final result will be M|R|L
    /// Reverse the result, become L | R | M, that is what we want
    fn postorder_traverse(root: Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        let mut stack = Vec::new();
        stack.push(root.clone());
        while let Some(node) = stack.pop() {
            if let Some(n) = node {
                res.push(n.borrow().val.clone());
                stack.push(n.borrow().left.clone());
                stack.push(n.borrow().right.clone());
            }
        }
        res.reverse();
    }

    fn inorder_traverse_alter(root: Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        let mut stack = Vec::new();
        stack.push((root.clone(), false)); // false to indicate not all its children has not been visited
        while let Some((node, visited)) = stack.pop() {
            if visited {
                res.push(node.clone().unwrap().borrow().val.to_owned());
            } else {
                if let Some(cur) = node {
                    // push right node first
                    if cur.borrow().right.is_some() {
                        stack.push((cur.borrow().right.clone(), false));
                    }
                    // push self into the stack again
                    // right child has been pushed in to stack already
                    // set the visited as true
                    stack.push((Some(cur.clone()), true));
                    // push left node
                    if cur.borrow().left.is_some() {
                        stack.push((cur.borrow().left.clone(), false));
                    }
                }
            }
        }
    }

    fn preorder_traverse_alter(root: Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        let mut stack = Vec::new();
        stack.push((root.clone(), false));
        while let Some((node, visited)) = stack.pop() {
            if visited {
                res.push(node.clone().unwrap().borrow().val.to_owned());
            } else {
                if let Some(cur) = node {
                    // push the right node
                    if cur.borrow().right.is_some() {
                        stack.push((cur.borrow().right.clone(), false));
                    }
                    // push the left node
                    if cur.borrow().left.is_some() {
                        stack.push((cur.borrow().left.clone(), false));
                    }
                    // push the current back
                    stack.push((Some(cur.clone()), true));
                }
            }
        }
    }

    fn postorder_traverse_alter(root: Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
        let mut stack = Vec::new();
        stack.push((root.clone(), false));
        while let Some((node, visited)) = stack.pop() {
            if visited {
                res.push(node.clone().unwrap().borrow().val.to_owned());
            } else {
                if let Some(cur) = node {
                    // push current back
                    stack.push((Some(cur.clone()), true));
                    if cur.borrow().right.is_some() {
                        stack.push((cur.borrow().right.clone(), false));
                    }
                    if cur.borrow().left.is_some() {
                        stack.push((cur.borrow().left.clone(), false));
                    }
                }
            }
        }
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
        let root = Some(Rc::new(RefCell::new(node_1)));
        TreeNode::inorder_traverse(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
        TreeNode::preorder_traverse(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
        TreeNode::postorder_traverse(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();

        // alter way
        TreeNode::inorder_traverse_alter(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
        TreeNode::preorder_traverse_alter(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
        TreeNode::postorder_traverse_alter(root.clone(), &mut ret);
        println!("{:?}", ret);
        ret.clear();
    }
}
