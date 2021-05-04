/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3731/
 */

pub struct Solution {}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        Self::check_in_sort(nums.clone()) || Self::check_reversed(nums.clone())
    }

    fn check_in_sort(mut nums: Vec<i32>) -> bool {
        let mut increase_exists = false;
        for i in 0..nums.len() - 1 {
            if nums[i] <= nums[i + 1] {
                continue;
            }
            if increase_exists {
                return false;
            }
            nums[i + 1] = nums[i];
            increase_exists = true;
        }
        true
    }

    fn check_reversed(mut nums: Vec<i32>) -> bool {
        let mut increase_exists = false;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] <= nums[i + 1] {
                continue;
            }
            if increase_exists {
                return false;
            }
            nums[i] = nums[i + 1];
            increase_exists = true;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![4, 2, 3];
        assert!(Solution::check_possibility(nums));
    }

    #[test]
    fn example_2() {
        let nums = vec![4, 2, 1];
        assert!(!Solution::check_possibility(nums));
    }

    #[test]
    fn submission_1() {
        let nums = vec![3, 4, 2, 3];
        assert!(!Solution::check_possibility(nums));
    }

    #[test]
    fn submission_2() {
        let nums = vec![5, 7, 1, 8];
        assert!(Solution::check_possibility(nums));
    }
}
