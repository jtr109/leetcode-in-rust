/*
 * [Running Sum of 1d Array](https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3730/)
 */

pub struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums;
        for i in 1..n.len() {
            n[i] += n[i - 1];
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![1, 2, 3, 4];
        let output = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(input), output);
    }

    #[test]
    fn test_example_2() {
        let input = vec![1, 1, 1, 1, 1];
        let output = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(input), output);
    }

    #[test]
    fn test_example_3() {
        let input = vec![3, 1, 2, 10, 1];
        let output = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(input), output);
    }
}
