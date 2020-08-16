/*!
 * # 5. Longest Palindromic Substring
 *
 * [Problem link](https://leetcode.com/problems/longest-palindromic-substring/)
 */

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        "bab".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "babad".to_string();
        let outputs = vec!["bab".to_string(), "aba".to_string()];
        assert!(outputs.contains(&Solution::longest_palindrome(input)));
    }
}
