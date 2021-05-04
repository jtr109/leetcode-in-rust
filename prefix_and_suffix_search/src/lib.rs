/*
 * [Explore - LeetCode](https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3728/)
 *
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

pub struct WordFilter {
    words: Vec<String>,
}

impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        WordFilter { words }
    }

    pub fn f(&self, prefix: String, suffix: String) -> i32 {
        'word_loop: for (index, word) in self.words.iter().enumerate().rev() {
            if word.len() < prefix.len().max(suffix.len()) {
                continue;
            }
            // query prefix
            let mut word_chars = word.chars();
            let mut prefix_chars = prefix.chars();
            while let Some(c) = prefix_chars.next() {
                if word_chars.next().unwrap() != c {
                    break 'word_loop;
                }
            }
            // query suffix
            let mut word_chars_rev = word.chars().rev();
            let mut suffix_chars_rev = suffix.chars().rev();
            while let Some(c) = suffix_chars_rev.next() {
                if word_chars_rev.next().unwrap() != c {
                    break 'word_loop;
                }
            }
            return index as i32;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let words = vec!["apple"].iter().map(|s| s.to_string()).collect();
        let prefix = "a".to_string();
        let suffix = "e".to_string();
        let expected = 0;
        let obj = WordFilter::new(words);
        assert_eq!(obj.f(prefix, suffix), expected);
    }

    #[test]
    fn submission_1() {
        let words = vec![
            "cabaabaaaa",
            "ccbcababac",
            "bacaabccba",
            "bcbbcbacaa",
            "abcaccbcaa",
            "accabaccaa",
            "cabcbbbcca",
            "ababccabcb",
            "caccbbcbab",
            "bccbacbcba",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
        let patterns = vec![
            vec!["bccbacbcba", "a"],
            vec!["ab", "abcaccbcaa"],
            vec!["a", "aa"],
            vec!["cabaaba", "abaaaa"],
            vec!["cacc", "accbbcbab"],
            vec!["ccbcab", "bac"],
            vec!["bac", "cba"],
            vec!["ac", "accabaccaa"],
            vec!["bcbb", "aa"],
            vec!["ccbca", "cbcababac"],
        ]
        .iter()
        .map(|p| vec![p[0].to_string(), p[1].to_string()])
        .collect::<Vec<Vec<String>>>();
        let result = patterns
            .iter()
            .map(|p| WordFilter::new(words.clone()).f(p[0].clone(), p[1].clone()))
            .collect::<Vec<i32>>();
        assert_eq!(result, vec![9, 4, 5, 0, 8, 1, 2, 5, 3, 1])
    }
}
