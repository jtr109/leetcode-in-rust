use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let head = vec![-10, -3, 0, 5, 9];
        let expected = vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)];
    }

    #[test]
    fn example_2() {
        let head: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
    }

    #[test]
    fn example_3() {
        let head = vec![0];
        let expected = vec![0];
    }

    #[test]
    fn example_4() {
        let head = vec![1, 3];
        let expected = vec![3, 1];
    }
}
