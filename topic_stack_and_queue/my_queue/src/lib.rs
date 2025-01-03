#[derive(Debug, Clone)]
struct MyQueue {
    stack_1: MyStack,
    stack_2: MyStack,
}

#[derive(Debug, Clone)]
struct MyStack {
    stack: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, num: i32) {
        self.stack.push(num);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn size(&self) -> usize {
        self.stack.len()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack_1: MyStack::new(),
            stack_2: MyStack::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_1.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut idx = self.stack_1.size();
        while idx > 1 {
            self.stack_2.push(self.stack_1.pop().unwrap());
            idx -= 1;
        }
        let ret = self.stack_1.pop().unwrap();
        while let Some(num) = self.stack_2.pop() {
            self.stack_1.push(num);
        }
        ret
    }

    fn peek(&mut self) -> i32 {
        while let Some(num) = self.stack_1.pop() {
            self.stack_2.push(num);
        }
        let ret = self.stack_2.pop().unwrap();
        self.stack_1.push(ret);
        while let Some(num) = self.stack_2.pop() {
            self.stack_1.push(num);
        }
        ret
    }

    fn empty(&self) -> bool {
        self.stack_1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false)
    }
}
