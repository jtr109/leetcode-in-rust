#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

struct Extractor {
    nums: Vec<i32>,
    nums_map: HashMap<i32, usize>,
    sums_map: HashMap<Vec<i32>, ()>,
}

impl Extractor {
    fn new(nums: &Vec<i32>) -> Self {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let mut nums_map = HashMap::new();
        for &n in nums.iter() {
            let count = nums_map.entry(n).or_insert(0);
            *count += 1;
        }
        Extractor {
            nums: sorted_nums,
            nums_map,
            sums_map: HashMap::new(),
        }
    }

    fn get_expected(&self, n: &i32, m: &i32) -> Option<i32> {
        let exp = 0 - n - m;
        let count = self.nums_map.get(&exp).unwrap_or(&0);
        if vec![*n, *m, exp].iter().filter(|&x| *x == exp).count() > *count {
            return None;
        }
        Some(exp)
    }

    fn build_sums_map(&mut self) {
        for (i, n) in self.nums.iter().enumerate() {
            'second_loop: for m in self.nums[i + 1..].iter() {
                if let Some(exp) = self.get_expected(n, m) {
                    if exp < *m {
                        // wrong sort
                        break 'second_loop;
                    } else {
                        self.sums_map.insert(vec![*n, *m, exp], ());
                    }
                } else {
                    continue 'second_loop;
                }
            }
        }
    }

    fn three_sum(&mut self) -> Vec<Vec<i32>> {
        self.build_sums_map();
        self.sums_map.keys().cloned().collect()
    }
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ext = Extractor::new(&nums);
        ext.three_sum()
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
