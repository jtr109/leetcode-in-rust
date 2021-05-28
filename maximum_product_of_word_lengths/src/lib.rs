pub struct Solution {}

impl Solution {
    fn cal_mask(word: &str) -> i32 {
        word.chars()
            .fold(0, |acc, x| acc | 1 << (x as u8 - 'a' as u8))
    }

    fn share_common_letters(mask_cache: &Vec<i32>, i1: usize, i2: usize) -> bool {
        let mask1 = mask_cache[i1];
        let mask2 = mask_cache[i2];
        mask1 & mask2 != 0
    }

    pub fn max_product(words: Vec<String>) -> i32 {
        let mut length_cache = Vec::with_capacity(words.len());
        let mut mask_cache = Vec::with_capacity(words.len());
        for word in words.iter() {
            length_cache.push(word.chars().count() as i32);
            mask_cache.push(Self::cal_mask(word));
        }
        let mut max = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if Self::share_common_letters(&mask_cache, i, j) {
                    continue;
                }
                max = max.max(length_cache[i] * length_cache[j]);
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
