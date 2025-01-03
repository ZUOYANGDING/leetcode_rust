use std::borrow::{Borrow, BorrowMut};

struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Self {
            val: -1,
            next: None,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }

        let mut cur = self.next.borrow();
        let mut count = 0;
        while let Some(node) = cur {
            if count == index {
                return node.val;
            } else {
                cur = node.next.borrow();
                count += 1;
            }
        }
        return -1;
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList {
            val,
            next: self.next.take(),
        });
        self.next = Some(new_node)
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList { val, next: None });
        let mut cur = self.next.borrow_mut();
        while let Some(node) = cur {
            cur = node.next.borrow_mut();
        }
        cur.replace(new_node);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val)
        } else {
            let mut cur = self.next.borrow_mut();
            let mut count = 0;
            while let Some(node) = cur {
                if count + 1 == index {
                    let new_node = Box::new(MyLinkedList {
                        val,
                        next: node.next.take(),
                    });
                    node.next = Some(new_node);
                    break;
                } else {
                    cur = node.next.borrow_mut();
                    count += 1;
                }
            }
        }
    }

    fn delete_at_index(&self, index: i32) {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
