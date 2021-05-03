/*
 * [Explore - LeetCode](https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3729/)
 */

use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        // order by deadline
        let mut courses = courses;
        courses.sort_by(|x, y| x[1].cmp(&y[1]));
        let mut deadline = 0;
        let mut selected = BinaryHeap::new();
        for c in courses.iter() {
            let duration = c[0];
            let last_day = c[1];
            if deadline + duration <= last_day {
                selected.push(c);
                deadline += duration;
                continue;
            }
            if selected.len() > 0
                && selected.peek().unwrap()[0] > duration
                && deadline - selected.peek().unwrap()[0] + duration <= last_day
            {
                let max = selected.pop().unwrap();
                selected.push(c);
                deadline = duration - max[0] + duration;
                continue;
            }
        }
        selected.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let courses = vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200],
        ];
        assert_eq!(Solution::schedule_course(courses), 3);
    }

    #[test]
    fn test_example_2() {
        let courses = vec![vec![1, 2]];
        assert_eq!(Solution::schedule_course(courses), 1);
    }

    #[test]
    fn test_example_3() {
        let courses = vec![vec![3, 2], vec![4, 3]];
        assert_eq!(Solution::schedule_course(courses), 0);
    }
}
