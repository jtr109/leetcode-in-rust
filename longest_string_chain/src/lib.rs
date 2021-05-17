pub struct Solution {}

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words = vec!["a", "b", "ba", "bca", "bda", "bdca"];
        let expected = 4;
        assert_eq!(
            Solution::longest_str_chain(words.iter().map(|x| x.to_string()).collect()),
            expected
        );
    }

    #[test]
    fn example_2() {
        let words = vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"];
        let expected = 5;
        assert_eq!(
            Solution::longest_str_chain(words.iter().map(|x| x.to_string()).collect()),
            expected
        );
    }
}
