/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
 */

pub struct Solution {}

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        let mut target = target;
        loop {
            let mut any_num_less_than_one = false;
            let mut all_nums_are_one = true;
            let mut max_index = 0;
            let mut second_max_index = 0;
            // TODO: second_max_index 算法有问题
            let mut sum = 0;
            for (i, &n) in target.iter().enumerate() {
                if !any_num_less_than_one && n < 1 {
                    any_num_less_than_one = true;
                }
                if all_nums_are_one && n != 1 {
                    all_nums_are_one = false;
                }
                sum += n;
                if n > target[max_index] {
                    second_max_index = max_index;
                    max_index = i;
                } else if n > target[second_max_index] {
                    second_max_index = i;
                }
            }
            if any_num_less_than_one {
                return false;
            }
            if all_nums_are_one {
                return true;
            }
            loop {
                // 如果 max_index 中新的数字还是大于第二大的数字，那么不需要循环，直接再次减去其他数字。
                // 取余数
                let others_sum = sum - target[max_index];
                target[max_index] -= others_sum;
                sum -= others_sum;
                if target[max_index] <= target[second_max_index] {
                    break;
                }
            }
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

    #[test]
    fn submission_1() {
        let target = vec![5, 2];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn submission_2() {
        let target = vec![9, 9, 9];
        assert!(!Solution::is_possible(target));
    }

    #[test]
    fn submission_3() {
        let target = vec![1, 1, 61, 9, 17];
        assert!(Solution::is_possible(target));
    }
}
