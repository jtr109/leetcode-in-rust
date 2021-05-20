/*!
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3749/
 */

use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        match root {
            None => vec![],
            Some(rt) => {
                let mut left = Self::level_order(rt.borrow().left.clone());
                let mut right = Self::level_order(rt.borrow().right.clone());
                let mut result = vec![];
                for i in 0..left.len().max(right.len()) + 1 {
                    result.push(vec![]);
                    if i == 0 {
                        result[i].push(rt.borrow().val);
                        continue;
                    }
                    let j = i - 1;
                    if j < left.len() {
                        result[i].append(&mut left[j]);
                    }
                    if j < right.len() {
                        result[i].append(&mut right[j]);
                    }
                }
                result
            }
        }
    }
}
