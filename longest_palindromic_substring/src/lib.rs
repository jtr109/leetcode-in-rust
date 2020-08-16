/*!
 * # 5. Longest Palindromic Substring
 *
 * [Problem link](https://leetcode.com/problems/longest-palindromic-substring/)
 */

#![allow(dead_code)]

struct Solution {}

// ----------------------------------------------------------------------------

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let odd_longest: String = {
            let mut longest_part = vec![];
            for (i, &c) in chars.iter().enumerate() {
                let mut current_part = vec![c];
                let (mut low, mut high) = (i, i);
                loop {
                    if chars[low] != chars[high] {
                        // not a palindrome
                        break;
                    }
                    if low != high {
                        // exclude the condition only one char
                        current_part.push(chars[low]);
                    }
                    if low > 0 && high < chars.len() - 1 {
                        low -= 1;
                        high += 1;
                    } else {
                        break;
                    }
                }
                if current_part.len() > longest_part.len() {
                    longest_part = current_part;
                }
            }
            let left_part = longest_part[1..]
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<char>>();
            [left_part, longest_part].concat().iter().cloned().collect()
        };
        let even_longest: String = {
            let mut longest_part = vec![];
            for (i, _) in chars[..chars.len() - 1].iter().enumerate() {
                let mut current_part = vec![];
                let (mut low, mut high) = (i, i + 1);
                loop {
                    if chars[low] == chars[high] {
                        current_part.push(chars[high]);
                    } else {
                        break;
                    }
                    if low > 0 && high < chars.len() - 1 {
                        low -= 1;
                        high += 1;
                    } else {
                        break;
                    }
                }
                if current_part.len() > longest_part.len() {
                    longest_part = current_part;
                }
            }
            let left_part = longest_part.iter().rev().cloned().collect::<Vec<char>>();
            [left_part, longest_part].concat().iter().cloned().collect()
        };
        if odd_longest.len() > even_longest.len() {
            odd_longest
        } else {
            even_longest
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted(input: String, expected: Vec<String>) {
        assert!(expected.contains(&Solution::longest_palindrome(input)));
    }

    #[test]
    fn test_example_1() {
        let input = "babad".to_string();
        let expected = vec!["bab".to_string(), "aba".to_string()];
        accepted(input, expected);
    }

    #[test]
    fn test_example_2() {
        let input = "cbbd".to_string();
        let expected = vec!["bb".to_string()];
        accepted(input, expected);
    }

    #[test]
    fn test_empty_string() {
        let input = "".to_string();
        let expected = vec!["".to_string()];
        accepted(input, expected);
    }

    #[test]
    fn test_one_string() {
        let input = "a".to_string();
        let expected = vec!["a".to_string()];
        accepted(input, expected);
    }

    #[test]
    fn test_two_string() {
        let input = "bb".to_string();
        let expected = vec!["bb".to_string()];
        accepted(input, expected);
    }
}
