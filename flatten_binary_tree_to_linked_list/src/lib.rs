use std::cell::RefCell;
use std::rc::Rc;

/// Definition for a binary tree node.
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match root {
            None => (),
            Some(r) => {
                Self::flatten(&mut r.borrow_mut().left);
                Self::flatten(&mut r.borrow_mut().right);
                match r.borrow().left.clone() {
                    None => return,
                    Some(l) => {
                        let mut node = l;
                        while node.borrow().right.is_some() {
                            let node_right = node.borrow().right.clone();
                            node = node_right.unwrap();
                        }
                        node.borrow_mut().right = r.borrow().right.clone();
                    }
                }
                let left = r.borrow().left.clone();
                r.borrow_mut().right = left;
                r.borrow_mut().left = None;
            }
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        Solution::flatten(&mut root);
        assert_eq!(root.clone().unwrap().borrow_mut().val, 1);
        assert_eq!(root.clone().unwrap().borrow_mut().left, None);
        let r1 = root.clone().unwrap().borrow_mut().right.clone();
        assert_eq!(r1.clone().unwrap().borrow_mut().val, 2);
        assert_eq!(r1.clone().unwrap().borrow_mut().left, None);
        let r2 = r1.clone().unwrap().borrow_mut().right.clone();
        assert_eq!(r2.clone().unwrap().borrow_mut().val, 3);
        assert_eq!(r2.clone().unwrap().borrow_mut().left, None);
        let r3 = r2.clone().unwrap().borrow_mut().right.clone();
        assert_eq!(r3.clone().unwrap().borrow_mut().val, 4);
        assert_eq!(r3.clone().unwrap().borrow_mut().left, None);
        let r4 = r3.clone().unwrap().borrow_mut().right.clone();
        assert_eq!(r4.clone().unwrap().borrow_mut().val, 5);
        assert_eq!(r4.clone().unwrap().borrow_mut().left, None);
        let r5 = r4.clone().unwrap().borrow_mut().right.clone();
        assert_eq!(r5.clone().unwrap().borrow_mut().val, 6);
        assert_eq!(r5.clone().unwrap().borrow_mut().left, None);
        assert_eq!(r5.clone().unwrap().borrow_mut().right, None);
    }
}
