pub struct Solution {}

impl Solution {
    fn share_common_letters(w1: &String, w2: &String) -> bool {
        for c1 in w1.chars() {
            for c2 in w2.chars() {
                if c1 == c2 {
                    return true;
                }
            }
        }
        false
    }

    pub fn max_product(words: Vec<String>) -> i32 {
        let mut max = 0;
        for (i, w1) in words.iter().enumerate() {
            for j in i + 1..words.len() {
                let w2 = &words[j];
                if Self::share_common_letters(w1, w2) {
                    continue;
                }
                max = max.max((w1.chars().count() * w2.chars().count()) as i32);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works(words: Vec<&str>, expected: i32) {
        assert_eq!(
            Solution::max_product(words.iter().map(|x| x.to_string()).collect()),
            expected
        );
    }

    #[test]
    fn example_1() {
        let words = vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
        let expected = 16;
        it_works(words, expected);
    }

    #[test]
    fn example_2() {
        let words = vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
        let expected = 4;
        it_works(words, expected);
    }

    #[test]
    fn example_3() {
        let words = vec!["a", "aa", "aaa", "aaaa"];
        let expected = 0;
        it_works(words, expected);
    }
}
