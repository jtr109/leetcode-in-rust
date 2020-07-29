#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sums = vec![];
        let mut sorted = nums.clone();
        sorted.sort();

        for i in 0..sorted.len() {
            let n = sorted[i];
            if i > 0 && sorted[i - 1] == n {
                continue;
            }
            if n > 0 {
                break;
            }
            let (mut low, mut high) = (i + 1, sorted.len() - 1);
            while low < high {
                let group = vec![n, sorted[low], sorted[high]];
                let s: i32 = group.iter().sum();
                if s < 0 {
                    low += 1;
                    continue;
                } else if s > 0 {
                    high -= 1;
                    continue;
                }
                sums.push(group);
                while low < high && sorted[low] == sorted[low + 1] {
                    low += 1;
                }
                while low < high && sorted[high] == sorted[high - 1] {
                    high -= 1;
                }
                low += 1;
                high -= 1;
            }
        }

        sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
