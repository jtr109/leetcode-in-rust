pub struct Solution {}

impl Solution {
    fn share_common_letters(w1: &String, w2: &String) -> bool {
        let mut mask1 = 0;
        for c1 in w1.chars() {
            mask1 |= 1 << (c1 as u8 - 'a' as u8);
        }
        let mut mask2 = 0;
        for c2 in w2.chars() {
            mask2 |= 1 << (c2 as u8 - 'a' as u8);
        }
        mask1 & mask2 != 0
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
