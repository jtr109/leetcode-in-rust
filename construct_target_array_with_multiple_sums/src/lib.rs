/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
 */

pub struct Solution {}

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut target = target;
        loop {
            let mut any_num_less_than_one = false;
            let mut all_nums_are_one = true;
            let mut max_num = 0;
            let mut max_index = 0;
            let mut sum = 0;
            for (i, &n) in target.iter().enumerate() {
                if !any_num_less_than_one && n < 1 {
                    any_num_less_than_one = true;
                }
                if all_nums_are_one && n != 1 {
                    all_nums_are_one = false;
                }
                sum += n;
                if max_num < n {
                    max_index = i;
                    max_num = n;
                }
            }
            if any_num_less_than_one {
                return false;
            }
            if all_nums_are_one {
                return true;
            }
            target[max_index] = 2 * target[max_index] - sum;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let target = vec![9, 3, 5];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn example_2() {
        let target = vec![1, 1, 1, 2];
        assert!(!Solution::is_possible(target));
    }

    #[test]
    fn example_3() {
        let target = vec![8, 5];
        assert!(Solution::is_possible(target));
    }
}
