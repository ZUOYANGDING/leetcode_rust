### Understand Box
- Box\<T> is a smart pointer which **OWNED** the T, and point to address of the T on heap
  - Box\<T> take ownership of the memory which contains T on heap, and any assignment statement like "let a = Box\<T>" will move the ownership. If need to share the ownership without move it, use Rc to share
  - Create Box\<T> usually been operated as following by complier:
    - allocate the memory of the value on stack
    - start "Boxing":
      - allocate memory on heap
      - copy the value from stack to heap
      - store the pointer point to the memory on heap to stack
- The value of Box\<T> is the address of the T on heap
- Get the address of Box it self on **stack** and address of T on **heap**
  ```
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ListNode {
        val: i32,
        next: Option<Box<ListNode>>,
    }
    impl ListNode {
        fn new(val: i32) -> ListNode {
            Self { val, next: None }
        }
    }
    fn build_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
    fn print_address() {
        let common = build_list(&[10, 9, 8]);
        let stack_address = &(common.as_ref().unwrap()) as *const _;
        let heap_address = common.as_deref().unwrap() as *const _;
        println!("stack_address {:p}", stack_address);
        println!("heap_addrsss {:p}", heap_address); 
    }

  ```
- Clone of Box\<T>
  - Usually, Box\<T>.clone() will trigger a **deep copy**, which means creates a new Box instance, which allocates a new block of heap memory for the cloned Box contents.
  - However, if do Box\<T>.clone().clone(), the second one might do not trigger the **deep copy**, which means only a new Box instance created by no new block of heap memory created. This situation depends on how does the compiler works.
  Here is an example: 
  ```
    fn test_box_memory_allocation() {
        let node = build_list(&[10, 9, 8]);
        let node_1 = node.clone();
        let node_2 = node.clone();
        let node_3 = node.clone();
        println!(
            "ListNode address on heap  {:p}, value: {:?}",
            node.as_deref().unwrap() as *const _,
            node.as_deref().unwrap().val
        );
        println!("{:p}", node.unwrap());
        println!(
            "ListNode address on heap  {:p}, value: {:?}",
            node_1.as_deref().unwrap() as *const _,
            node_1.as_deref().unwrap().val
        );
        println!("{:p}", node_1.unwrap());
        println!(
            "ListNode address on heap  {:p}, value: {:?}",
            node_2.as_deref().unwrap() as *const _,
            node_2.as_deref().unwrap().val
        );
        println!("{:p}", node_2.unwrap());
        println!(
            "ListNode address on heap  {:p}, value: {:?}",
            node_3.as_deref().unwrap() as *const _,
            node_3.as_deref().unwrap().val
        );
        println!("{:p}", node_3.unwrap());
        println!("===================================");
        let node = build_list(&[10, 9, 8]);
        let node_1 = node.clone();
        let node_2 = node.clone();
        let node_3 = node.clone();
        println!(
            "ListNode address on heap {:p}, value: {:?}",
            node.as_deref().unwrap() as *const _,
            node.as_deref().unwrap().val
        );
        println!("{:p}", node.unwrap());
        println!(
            "ListNode address on heap {:p}, value:  {:?}",
            node_1.as_deref().unwrap() as *const _,
            node_1.as_deref().unwrap().val
        );
        println!("{:p}", node_1.clone().unwrap());
        println!(
            "ListNode address on heap {:p}, value: {:?}",
            node_2.as_deref().unwrap() as *const _,
            node_2.as_deref().unwrap().val
        );
        println!("{:p}", node_2.clone().unwrap());
        println!(
            "ListNode address on heap {:p}, value: {:?}",
            node_3.as_deref().unwrap() as *const _,
            node_3.as_deref().unwrap().val
        );
        println!("{:p}", node_3.clone().unwrap());
    }

    And result is 
    ---- tests::test_box_memory_allocation stdout ----
    ListNode address on heap  0x1217040f0, value: 8
    0x1217040f0
    ListNode address on heap  0x121706a30, value: 8
    0x121706a30
    ListNode address on heap  0x121706a60, value: 8
    0x121706a60
    ListNode address on heap  0x121706a90, value: 8
    0x121706a90
    ===================================
    ListNode address on heap 0x121706aa0, value: 8
    0x121706aa0
    ListNode address on heap 0x121706ab0, value:  8
    0x121706aa0
    ListNode address on heap 0x121706a30, value: 8
    0x121706aa0
    ListNode address on heap 0x121706a60, value: 8
    0x121706aa0
  ```
- AsRef and DeRef
  - Box::as_ref() get reference of the T
  - Box::deref() get reference of the T
  - which means if use "println!{"{:p}"} to print the result of as_ref() and deref(), both of them will print the address of T on heap
    - Same as before, if the T is not a basic type, even the result of as_ref() and deref() will provide a reference of the T, but the address is not exactly the address that T stored on heap. 
- Here is a more complicated example:
  ```
  #[test]
  fn test_intersaction() {
      let common = build_list(&[10, 9, 8]);
      let stack_address = &(common.as_ref().unwrap()) as *const _;
      let heap_address = common.as_deref().unwrap() as *const _;
      println!("stack_address {:p}", stack_address);
      println!("heap_addrsss {:p}", heap_address);
      // build A
      let mut head_a = build_list(&[4, 1]);
      if let Some(node) = head_a.as_mut() {
          node.next.as_mut().unwrap().next = common.clone();
      }
      println!("{:?}", head_a);
      println!(
          "heap address: {:p}, value: {}",
          head_a.as_deref().unwrap() as *const _,
          head_a.as_deref().unwrap().val
      );
      println!(
          "heap address: {:p}, value: {}",
          head_a.as_deref().unwrap().next.as_deref().unwrap() as *const _,
          head_a.as_deref().unwrap().next.as_deref().unwrap().val
      );
      println!(
          "heap address: {:p}, value: {}",
          head_a
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap() as *const _,
          head_a
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .val
      );
      // biuld B
      let mut head_b = build_list(&[5, 6, 1]);
      if let Some(node) = head_b.as_mut() {
          node.next.as_mut().unwrap().next.as_mut().unwrap().next = common.clone();
      }
      println!("{:?}", head_b);
      println!(
          "heap address {:p}, value: {}",
          head_b.as_deref().unwrap() as *const _,
          head_b.as_deref().unwrap().val
      );
      println!(
          "heap address {:p}, value: {}",
          head_b.as_deref().unwrap().next.as_deref().unwrap() as *const _,
          head_b.as_deref().unwrap().next.as_deref().unwrap().val
      );
      println!(
          "heap address {:p}, value {}",
          head_b
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap() as *const _,
          head_b
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .val
      );
      println!(
          "heap address {:p}, value: {}",
          head_b
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap() as *const _,
          head_b
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .next
              .as_deref()
              .unwrap()
              .val
      );
      let result = Solution::intersaction_node(head_a, head_b);
      assert_eq!(result, common);
  }

  ---- tests::test_intersaction stdout ----
  stack_address 0x16d6b5cf0
  heap_addrsss 0x11ee062f0
  Some(ListNode { val: 1, next: Some(ListNode { val: 4, next: Some(ListNode { val: 8, next: Some(ListNode { val: 9, next: Some(ListNode { val: 10, next: None }) }) }) }) })
  heap address: 0x11ee08bf0, value: 1
  heap address: 0x11ee06300, value: 4
  heap address: 0x11ee08c00, value: 8
  Some(ListNode { val: 1, next: Some(ListNode { val: 6, next: Some(ListNode { val: 5, next: Some(ListNode { val: 8, next: Some(ListNode { val: 9, next: Some(ListNode { val: 10, next: None }) }) }) }) }) })
  heap address 0x11ee08df0, value: 1
  heap address 0x11ee08510, value: 6
  heap address 0x11ee08500, value 5
  heap address 0x11ee08e00, value: 8
  ```
  The above example indicate serveral factors:
    - How to get the address of Box itself on stack, and how to get the address of Box's content's address on heap
    - Box does not support ownership sharing, since the common ListNode address on heap is different to each other for head_a and head_b
    - Normally, clone of a Box will trigger the deep copy of the Box and its content


### Understand Rc
- Rc\<T> Smart pointer, which provide reference count of owners who share the memory of T
- T in Rc\<T> is **immutable**
- Get the address of the T on heap: "Rc\<T>::deref() as *const _"
- To increase the owner of the T, Rc\<T>::clone(&T);
- To check if T1 and T2 use same address on heap Rc\<T1> and Rc\<T2>, use Rc::eq_ptr()

### Understand RefCell
- RefCell\<T> is a Cell which provide mutable/imutable borrow of T, but only check at runtime (as_mut()/as_ref()/borrow()/borrow_mut())
- Similar to Box\<T>, RefCell\<T> only support single owner of T
- Since support borrow check at runtime, T in RefCell\<T> supppot mutable in some block but immuatable at other block
- Get the address of T in RefCell\<T>: "RefCell\<T>.as_ptr()


### Create a LinkedList
- No sharing of node 
  ```
  #[derive(Debug, Clone, PartialEq, Eq)]
  struct ListNode {
      val: i32,
      next: Option<Box<ListNode>>,
  } 
  ```
- Sharing of node (like double linked list or single linked list with loop)
  ```
  #[derive(Debug, PartialEq, Eq)]
  struct ListNode {
      val: i32,
      next: Option<Rc<RefCell<ListNode>>>,
  }
  or 
  #[derive(Debug, PartialEq, Eq)]
  struct ListNode {
      val: i32,
      next: Option<Rc<Box<ListNode>>>,
  } 
  ```
- complete exmaple: in "intersaction_node" lib
  
