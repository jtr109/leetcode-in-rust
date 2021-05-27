pub struct Solution {}

impl Solution {
    fn share_common_letters(
        mask_cache: &mut Vec<i32>,
        i1: usize,
        w1: &String,
        i2: usize,
        w2: &String,
    ) -> bool {
        let mask1 = match mask_cache.get(i1) {
            Some(&x) if x != 0 => x,
            _ => {
                let mask = w1
                    .chars()
                    .fold(0, |acc, x| acc | 1 << (x as u8 - 'a' as u8));
                mask_cache[i1] = mask;
                mask
            }
        };
        let mask2 = match mask_cache.get(i2) {
            Some(&x) if x != 0 => x,
            _ => {
                let mask = w2
                    .chars()
                    .fold(0, |acc, x| acc | 1 << (x as u8 - 'a' as u8));
                mask_cache[i2] = mask;
                mask
            }
        };
        mask1 & mask2 != 0
    }

    // fn get_length(length_cache: &Vec<usize>, index: usize, word: &String) -> usize {
    //     match length_cache.get(index) {
    //         Some(&l) if l != 0 {
    //             l
    //         },
    //         _ => {
    //             let length = word.chars().count();
    //             length_cache[]
    //         }
    //     }
    // }

    pub fn max_product(words: Vec<String>) -> i32 {
        let mut mask_cache = vec![0; words.len()];
        let mut max = 0;
        for (i, w1) in words.iter().enumerate() {
            for j in i + 1..words.len() {
                let w2 = &words[j];
                if Self::share_common_letters(&mut mask_cache, i, w1, j, w2) {
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
