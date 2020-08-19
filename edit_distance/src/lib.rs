/*!
 * # 72. Edit Distance
 *
 * [Problem link](https://leetcode.com/problems/edit-distance/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        3
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
