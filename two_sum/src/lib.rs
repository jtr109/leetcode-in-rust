/*!
 * # 1. Two Sum
 *
 * * [Problem link](https://leetcode.com/problems/two-sum/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------
#[derive(Eq, Ord, PartialOrd, PartialEq)]
struct Number {
    index: usize,
    value: i32,
}

impl Number {
    fn new(index: usize, value: i32) -> Self {
        Number { index, value }
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted = nums
            .iter()
            .enumerate()
            .map(|(index, &value)| Number::new(index, value))
            .collect::<Vec<Number>>();
        sorted.sort_by(|a, b| a.value.cmp(&b.value));
        let (mut low, mut high) = (0, sorted.len() - 1);
        while low < high {
            let s = sorted[low].value + sorted[high].value;
            if s == target {
                return vec![sorted[low].index as i32, sorted[high].index as i32];
            } else if s < target {
                low += 1;
            } else {
                high -= 1;
            }
        }
        panic!("no match");
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
