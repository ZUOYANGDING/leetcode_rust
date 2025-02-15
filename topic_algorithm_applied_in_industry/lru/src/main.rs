/**
 * Leetcode 146
 * Basic Idea:
 *      maintain a double linked list (ring buffer)
 *      use a HashMap<key, Node addr> to make the `get` and `remove` to O(1)
 */
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
struct Node {
    key: i32,
    val: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: Option<i32>, val: Option<i32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            key: match key {
                Some(k) => k,
                None => -1,
            },
            val: match val {
                Some(v) => v,
                None => -1,
            },
            prev: None,
            next: None,
        }))
    }

    fn push_front(dummy_head: Rc<RefCell<Node>>, node: Rc<RefCell<Node>>) {
        // when add in the first node
        // next == dummy_head
        let next = dummy_head.borrow().next.clone().unwrap();
        node.borrow_mut().prev = Some(Rc::downgrade(&dummy_head.clone()));
        node.borrow_mut().next = Some(next.clone());
        dummy_head.borrow_mut().next = Some(node.clone());
        // when add in the first node
        // it actually dummy_head.prev = node
        // the ring has been build with the dummy_head
        next.borrow_mut().prev = Some(Rc::downgrade(&node.clone()));
    }

    fn remove(node: Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.clone().unwrap().upgrade().unwrap();
        let next = node.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&prev.clone()));
    }
}

#[derive(Debug, Clone)]
struct LRUCache {
    dummy_head: Rc<RefCell<Node>>,
    key_map: HashMap<i32, Rc<RefCell<Node>>>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let mut key_map = HashMap::new();
        key_map.reserve(capacity as usize);
        let dummy_head = Node::new(None, None);
        dummy_head.borrow_mut().prev = Some(Rc::downgrade(&dummy_head.clone()));
        dummy_head.borrow_mut().next = Some(dummy_head.clone());
        Self {
            dummy_head,
            key_map,
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.key_map.get(&key) {
            let ret = node.borrow().val;
            Node::remove(node.clone());
            Node::push_front(self.dummy_head.clone(), node.clone());
            ret
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_map.get(&key) {
            Node::remove(node.clone());
            node.borrow_mut().val = value;
            Node::push_front(self.dummy_head.clone(), node.clone());
        } else {
            let node = Node::new(Some(key), Some(value));
            Node::push_front(self.dummy_head.clone(), node.clone());
            if self.key_map.len() == self.capacity {
                let back_node = self
                    .dummy_head
                    .borrow()
                    .prev
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                Node::remove(back_node.clone());
                self.key_map.remove(&back_node.borrow().key);
            }
            self.key_map.insert(key, node);
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
