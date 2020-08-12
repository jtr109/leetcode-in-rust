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
        let mut sorted1 = nums1;
        let mut sorted2 = nums2;
        sorted1.sort();
        sorted2.sort();
        let (mut i, mut j) = (0, 0);
        let mut inter = vec![];
        while i < sorted1.len() && j < sorted2.len() {
            let n1 = sorted1[i];
            let n2 = sorted2[j];
            if n1 < n2 {
                i += 1;
            } else if n1 > n2 {
                j += 1;
            } else {
                inter.push(n1);
                i += 1;
                j += 1;
                while i < sorted1.len() && sorted1[i] == n1 {
                    i += 1;
                }
                while j < sorted2.len() && sorted2[j] == n1 {
                    j += 1;
                }
            }
        }
        inter
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
