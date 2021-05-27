pub struct Solution {}

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {}
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
