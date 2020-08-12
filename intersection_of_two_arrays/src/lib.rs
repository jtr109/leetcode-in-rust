/*!
 * # 349. Intersection of Two Arrays
 *
 * * [Problem link](https://leetcode.com/problems/intersection-of-two-arrays/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn assert_equal(left: Vec<i32>, right: Vec<i32>) {
        let l = left.iter().cloned().collect::<HashSet<i32>>();
        let r = right.iter().cloned().collect::<HashSet<i32>>();
        assert_eq!(l, r);
    }

    #[test]
    fn test_example1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let expected = vec![2];
        assert_equal(Solution::intersection(nums1, nums2), expected);
    }

    #[test]
    fn test_example2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let expected = vec![9, 4];
        assert_equal(Solution::intersection(nums1, nums2), expected);
    }
}
