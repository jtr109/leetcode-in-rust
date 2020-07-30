#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let expected = 5;
        assert_eq!(expected, Solution::find_kth_largest(nums, k));
    }

    #[test]
    fn test_example2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let expected = 4;
        assert_eq!(expected, Solution::find_kth_largest(nums, k));
    }
}
