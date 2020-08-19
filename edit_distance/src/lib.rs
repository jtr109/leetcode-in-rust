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
        if word1.len() == 0 {
            return word2.len() as i32;
        }
        if word2.len() == 0 {
            return word1.len() as i32;
        }
        let w1 = word1[1..].to_string();
        let w2 = word2[1..].to_string();
        if word1.chars().next().unwrap() == word2.chars().next().unwrap() {
            return Solution::min_distance(w1.clone(), w2.clone());
        }
        let insert = 1 + Solution::min_distance(word1.clone(), w2.clone()); // insert the first char of word2 in front of word1
        let delete = 1 + Solution::min_distance(w1.clone(), word2.clone()); // delete the first char of word1
        let edit = 1 + Solution::min_distance(w1.clone(), w2.clone()); // change from the first char of word1 to the one of word2
        *[insert, delete, edit].iter().min().unwrap()
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
