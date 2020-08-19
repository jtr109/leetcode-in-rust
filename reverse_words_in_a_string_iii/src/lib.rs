/*!
 * # 557. Reverse Words in a String III
 *
 * [Problem link](https://leetcode.com/problems/reverse-words-in-a-string-iii/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn reverse_words(s: String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "Let's take LeetCode contest".to_string();
        let output = "s'teL ekat edoCteeL tsetnoc".to_string();
        assert_eq!(Solution::reverse_words(input), output);
    }
}
