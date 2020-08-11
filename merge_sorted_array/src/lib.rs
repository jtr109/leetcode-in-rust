/*!
 * 88. Merge Sorted Array
 *
 * * [Problem link](https://leetcode.com/problems/merge-sorted-array/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut length) = (0 as usize, m as usize);
        let mut j = 0 as usize;
        while j < n as usize {
            let n2 = nums2[j];
            if i >= length {
                // if all numbers in nums1 spent
                nums1[i] = n2;
                i += 1;
                length += 1;
                j += 1;
                continue;
            }
            if nums1[i] <= n2 {
                i += 1;
            } else {
                // insert n2
                let mut k = length;
                while k > i {
                    nums1[k] = nums1[k - 1];
                    k -= 1;
                }
                nums1[i] = n2;
                i += 1;
                length += 1;
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        let expected = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(nums1, expected);
    }

    #[test]
    fn test_example2() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        let expected = vec![1];
        assert_eq!(nums1, expected);
    }

    #[test]
    fn test_example3() {
        let mut nums1 = vec![2, 0];
        let m = 1;
        let mut nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        let expected = vec![1, 2];
        assert_eq!(nums1, expected);
    }
}
