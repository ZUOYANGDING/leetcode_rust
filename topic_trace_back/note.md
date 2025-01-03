#### Back Tracing
##### What is back tracing
Back tracing is a way to enum search all possible solution

##### Problems which back tracing can solve
1. Combination: find k numbers by some principle from N numbers
2. Slicing String: provided a principle, find how many slicing ways exsists 
3. Subset: find how many subsets matching requirement exists in a N numbers set
4. Permutation problem
5. Chase problem: N Queue...

##### Basic idea
Abstract the searching into a tree, then dealing with all possible nodes by recursion
Applied as DFS/BFS

##### Template
```Rust
fn backtracing(param) {
    if stop condition [
        save the result
        return;
    ]
    for element in set of elements in current layer {
        dealing with the element
        backtracing(branch, selection list)
        reverse the prev dealing process
    }
}
```