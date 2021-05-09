/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
 */

pub struct Solution {}

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut target = target;
        while !target.iter().any(|&x| x < 1) {
            if target.iter().all(|&x| x == 1) {
                return true;
            }
            let (max_index, &max_num) = target.iter().enumerate().max_by_key(|(_, &x)| x).unwrap();
            target[max_index] = 2 * max_num - target.iter().sum::<i32>();
        }
        false
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
