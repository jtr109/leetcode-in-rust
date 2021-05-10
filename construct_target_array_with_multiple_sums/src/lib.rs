/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
 */

use std::collections::BinaryHeap;

pub struct Solution {}

impl<'a> Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            match target.iter().nth(0) {
                Some(1) => return true,
                _ => return false,
            }
        }
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for e in target.iter() {
            heap.push(*e);
            sum += e;
        }
        loop {
            // 提取堆中最大的数字
            let mut max = heap.pop().unwrap();
            if max == 1 {
                return true;
            }
            let second_max = *heap.peek().unwrap();
            loop {
                // 处理最大数字直到它不是最大的
                let stuff = 2 * max - sum; // 计划插入对象
                if stuff < 1 {
                    return false;
                } else if stuff == 1 && second_max == 1 {
                    return true;
                }
                sum -= max - stuff;
                if stuff < second_max {
                    heap.push(stuff);
                    break;
                }
                max = stuff;
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

    #[test]
    fn submission_4() {
        let target = vec![2];
        assert!(!Solution::is_possible(target));
    }
}
