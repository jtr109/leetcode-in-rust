#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();
        let mut sums = vec![];
        for i in 0..sorted.len() {
            let mut j = i + 1;
            let mut k = sorted.len() - 1;
            while j < k {
                let group = vec![sorted[i], sorted[j], sorted[k]];
                match group.iter().sum::<i32>() {
                    s if s < 0 => j += 1,
                    s if s > 0 => k -= 1,
                    _ => {
                        sums.push(group);
                        break;
                    }
                }
            }
        }
        sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let given = vec![-1, 0, 1, 2, -1, -4];
        let mut expected: Vec<Vec<i32>> = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
        let mut got = Solution::three_sum(given);
        expected.sort();
        got.sort();
        assert_eq!(got, expected);
    }
}
