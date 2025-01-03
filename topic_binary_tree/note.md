### Tree

#### Data structure to represent Tree
- Array: if father node with index i, left child's index will be 2*i+1, and right child's index will be 2*i+2
- List

#### Binary Tree type
- Full binary tree: depth k binary tree with 2^k-1 nodes (k start from 1)
- Complete binary tree: Every layer is full of nodes except the deepest layer. Additionally, if the deepest layer does not have 2^(k-1) nodes, all the left child is not null
- Binary search tree: ordered nodes in tree: 
  - left child < father < right child
  - every left and right child also a binary search tree
- Balanced Binary search tree (AVL):
  - the diff of depth of left and right child no larger than 1
  - empty tree is also a AVL

#### Defined a binary tree
``` rust
#[derive(Debug, PartialEq, Eq)]
struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>
    pub right: Option<Rc<RefCell<TreeNode<T>>>>
}
impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None
        }
    }
}
```

#### Traverse
For a tree like 
```
                        1
                2               3
            4       5               6
```
- BFS: layer by layer
- DFS
  - Inorder: left -> root -> right (4, 2, 5, 1, 3, 6)
  - Preorder: root -> left -> right (1, 2, 4, 5, 3, 6)
  - Postorder: left -> right -> root (4, 5, 2 , 6, 3, 1) 

