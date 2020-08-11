/*!
 * # 232. Implement Queue using Stacks
 *
 * * [Problem link](https://leetcode.com/problems/implement-queue-using-stacks/)
 */

#![allow(dead_code)]

struct MyQueue {
    start: usize,
    data: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            start: 0,
            data: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    fn pop(&mut self) -> i32 {
        let v = self.data[self.start];
        self.start += 1;
        v
    }

    fn peek(&self) -> i32 {
        self.data[self.start]
    }

    fn empty(&self) -> bool {
        self.start == self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
    }
}
