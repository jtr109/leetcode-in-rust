/*!
 * # 72. Edit Distance
 *
 * [Problem link](https://leetcode.com/problems/edit-distance/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn reduce_min_distance(
        word1: &str,
        word2: &str,
        cache: &mut HashMap<(String, String), usize>,
    ) -> usize {
        if let Some(&min) = cache.get(&(word1.to_string(), word2.to_string())) {
            return min;
        };
        let (length1, length2) = (word1.len(), word2.len());
        if length1 == 0 || length2 == 0 {
            let min = length1.max(length2);
            cache.insert((word1.to_string(), word2.to_string()), min);
            return min;
        }
        let word1_slice = word1[1..].to_string();
        let word2_slice = word2[1..].to_string();
        if word1.chars().next().unwrap() == word2.chars().next().unwrap() {
            return Solution::reduce_min_distance(&word1_slice, &word2_slice, cache);
        }
        let insert = 1 + Solution::reduce_min_distance(word1, &word2_slice, cache); // insert the first char of word2 in front of word1
        let delete = 1 + Solution::reduce_min_distance(&word1_slice, word2, cache); // delete the first char of word1
        let edit = 1 + Solution::reduce_min_distance(&word1_slice, &word2_slice, cache); // change from the first char of word1 to the one of word2
        let min = *[insert, delete, edit].iter().min().unwrap();
        cache.insert((word1.to_string(), word2.to_string()), min);
        min
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut cache = HashMap::new();
        Self::reduce_min_distance(&word1, &word2, &mut cache) as i32
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn accept(word1: &str, word2: &str, expected: i32) {
        assert_eq!(
            Solution::min_distance(word1.to_string(), word2.to_string()),
            expected
        );
    }

    #[test]
    fn test_example_1() {
        let word1 = "horse";
        let word2 = "ros";
        let expected = 3;
        accept(word1, word2, expected);
    }

    #[test]
    fn test_example_2() {
        let word1 = "intention";
        let word2 = "execution";
        let expected = 5;
        accept(word1, word2, expected);
    }
}
