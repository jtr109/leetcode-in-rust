/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
 */

use std::collections::BinaryHeap;

pub struct Solution {}

impl<'a> Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for e in target.iter() {
            heap.push(*e);
            sum += e;
        }
        loop {
            let max = heap.pop().unwrap();
            if max == 1 {
                return true;
            }
            let stuff = 2 * max - sum;
            if stuff < 1 {
                return false;
            }
            sum -= max - stuff;
            heap.push(stuff);
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
