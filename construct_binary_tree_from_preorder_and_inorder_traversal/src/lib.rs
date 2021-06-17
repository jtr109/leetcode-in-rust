use std::cell::RefCell;
use std::ops::Index;
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match preorder.iter().nth(0) {
            None => None,
            Some(val) => {
                let index = inorder.iter().position(|v| val == v).unwrap();
                let left_inorder = inorder[..index].to_vec();
                let left_preorder = preorder[1..1 + index].to_vec();
                let right_inorder = inorder[index + 1..].to_vec();
                let right_preorder = preorder[1 + index..].to_vec();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: *val,
                    left: Self::build_tree(left_preorder, left_inorder),
                    right: Self::build_tree(right_preorder, right_inorder),
                })))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let preorder = [3, 9, 20, 15, 7];
        let inorder = [9, 3, 15, 20, 7];
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(
            Solution::build_tree(preorder.to_vec(), inorder.to_vec()),
            expected
        );
    }

    #[test]
    fn example_2() {
        let preorder = [-1];
        let inorder = [-1];
        let expected = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        assert_eq!(
            Solution::build_tree(preorder.to_vec(), inorder.to_vec()),
            expected
        );
    }
}
