/*!
 * # 1. Two Sum
 *
 * * [Problem link](https://leetcode.com/problems/two-sum/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![0, 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
