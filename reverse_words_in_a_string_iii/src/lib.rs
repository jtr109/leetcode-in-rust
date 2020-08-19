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
        let mut i = 0;
        let chars = s.chars().collect::<Vec<char>>();
        let mut reversed = vec![];
        while i < s.len() {
            if chars[i] == ' ' {
                reversed.push(chars[i]);
                i += 1;
                continue;
            }
            let mut j = i;
            while j < s.len() && chars[j] != ' ' {
                j += 1;
            }
            let mut k = j - 1;
            loop {
                // push the char into the reversed vector until reach the index `i`
                reversed.push(chars[k]);
                if k == i {
                    break;
                }
                k -= 1;
            }
            i = j;
        }
        reversed.iter().collect()
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
