/*!
 * # 145. Binary Tree Postorder Traversal
 *
 * * [problem link](https://leetcode.com/problems/binary-tree-postorder-traversal/)
 *
 * ## What is Postorder Traversal
 *
 * We can find explanation in [Wiki](https://en.wikipedia.org/wiki/Tree_traversal#Post-order_(LRN)):
 *
 * > Post-order (LRN)
 * >
 * > 1. Traverse the left subtree by recursively calling the post-order function.
 * > 2. Traverse the right subtree by recursively calling the post-order function.
 * > 3. Access the data part of the current node.
 */

#![allow(dead_code)]

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

// ----------------------------------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => return vec![],
            Some(n) => {
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();
                let val = n.borrow().val;
                let left_vec = Self::postorder_traversal(left);
                let right_vec = Self::postorder_traversal(right);
                let val_vec = vec![val];
                return [left_vec, right_vec, val_vec].concat();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let three = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let two = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: three,
            right: None,
        })));
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: two,
        })));
        let output = vec![3, 2, 1];
        assert_eq!(Solution::postorder_traversal(input), output);
    }
}
