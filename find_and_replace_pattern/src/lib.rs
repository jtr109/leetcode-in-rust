pub struct Solution {}

impl Solution {
    fn get_pattern(s: &str) -> Vec<Vec<usize>> {
        let mut pattern = vec![vec![]; 26];
        for (i, c) in s.char_indices() {
            pattern[c as usize - 'a' as usize].push(i);
        }
        pattern.sort();
        pattern.into_iter().filter(|x| x.len() > 0).collect()
    }

    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let p = Self::get_pattern(&pattern);
        words
            .into_iter()
            .filter(|w| Self::get_pattern(w) == p)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn matches(words: Vec<&str>, pattern: &str, expected: Vec<&str>) {
        assert_eq!(
            Solution::find_and_replace_pattern(
                words.iter().map(|x| x.to_string()).collect(),
                pattern.to_string(),
            ),
            expected
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        )
    }

    #[test]
    fn example_1() {
        let words = vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"];
        let pattern = "abb";
        let expected = vec!["mee", "aqq"];
        matches(words, pattern, expected);
    }

    #[test]
    fn example_2() {
        let words = vec!["a", "b", "c"];
        let pattern = "a";
        let expected = vec!["a", "b", "c"];
        matches(words, pattern, expected);
    }
}
