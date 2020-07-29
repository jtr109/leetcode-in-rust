#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();
        let mut num_map = HashMap::new();
        for (i, n) in sorted.iter().enumerate() {
            num_map.entry(n).or_insert(vec![]).push(i);
        }
        let mut sums = HashSet::new();

        for i in 0..sorted.len() {
            for j in (i + 2..sorted.len()).rev() {
                let n = sorted[i];
                let m = sorted[j];
                let expected = 0 - n - m;
                if let Some(indices) = num_map.get(&expected) {
                    if indices.iter().filter(|&&k| i < k && k < j).count() > 0 {
                        sums.insert(vec![n, expected, m]);
                    }
                }
            }
        }
        sums.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let given = vec![-1, 0, 1, 2, -1, -4];
        let expected: Vec<Vec<i32>> = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
        let got = Solution::three_sum(given);
        assert_eq!(
            got.iter().cloned().collect::<HashSet<Vec<i32>>>(),
            expected.iter().cloned().collect::<HashSet<Vec<i32>>>(),
        );
    }

    #[test]
    fn test_example2() {
        let given = vec![-2, 0, 1, 1, 2];
        let expected: Vec<Vec<i32>> = vec![vec![-2, 0, 2], vec![-2, 1, 1]];
        let got = Solution::three_sum(given);
        assert_eq!(
            got.iter().cloned().collect::<HashSet<Vec<i32>>>(),
            expected.iter().cloned().collect::<HashSet<Vec<i32>>>(),
        );
    }
}
