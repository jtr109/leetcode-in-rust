pub struct Solution {}

impl Solution {
    fn is_palindrome(word: &str) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            let j = chars.len() - 1 - i;
            if i >= j {
                break;
            }
            if chars[i] != chars[j] {
                return false;
            }
        }
        return true;
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for (i, w1) in words.iter().enumerate() {
            for (j, w2) in words.iter().enumerate() {
                if i == j {
                    continue;
                }
                let word = String::new() + w1 + w2;
                if Self::is_palindrome(&word) {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_expected(words: Vec<&str>, expected: Vec<[i32; 2]>) {
        let mut result = Solution::palindrome_pairs(words.iter().map(|x| x.to_string()).collect());
        result.sort();
        let mut expected_vector = expected
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        expected_vector.sort();
        assert_eq!(result, expected_vector);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome("abcddcba"));
        assert!(Solution::is_palindrome("dcbaabcd"));
        assert!(Solution::is_palindrome("slls"));
        assert!(Solution::is_palindrome("llssssll"));
        assert!(!Solution::is_palindrome("ssslllls"));
    }

    #[test]
    fn example_1() {
        let words = ["abcd", "dcba", "lls", "s", "sssll"];
        let expected = [[0, 1], [1, 0], [3, 2], [2, 4]];
        as_expected(words.to_vec(), expected.to_vec());
    }

    #[test]
    fn example_2() {
        let words = ["bat", "tab", "cat"];
        let expected = [[0, 1], [1, 0]];
        as_expected(words.to_vec(), expected.to_vec());
    }

    #[test]
    fn example_3() {
        let words = ["a", ""];
        let expected = [[0, 1], [1, 0]];
        as_expected(words.to_vec(), expected.to_vec());
    }
}
