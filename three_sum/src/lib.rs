#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut num_map = HashMap::new();
        for n in nums.iter() {
            *num_map.entry(n).or_insert(0) += 1;
        }
        let mut sums = HashSet::new();

        let keys: Vec<&i32> = num_map.keys().cloned().collect();
        for &&n in keys.iter() {
            for &&m in keys.iter() {
                let expected = 0 - m - n;
                let group = vec![n, m, expected];
                let mut group_map = HashMap::new();
                for k in group.iter() {
                    *group_map.entry(k).or_insert(0) += 1;
                }
                if group_map
                    .iter()
                    .all(|(&k, &count)| *num_map.get(k).unwrap_or(&0) >= count)
                {
                    let mut g = group.clone();
                    g.sort();
                    sums.insert(g);
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

    #[test]
    fn test_example3() {
        let given = vec![0, 0, 0];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        let got = Solution::three_sum(given);
        assert_eq!(
            got.iter().cloned().collect::<HashSet<Vec<i32>>>(),
            expected.iter().cloned().collect::<HashSet<Vec<i32>>>(),
        );
    }
}
