pub struct Solution {}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_expected(words: Vec<&str>, expected: Vec<[i32; 2]>) {
        let result = Solution::palindrome_pairs(words.iter().map(|x| x.to_string()).collect());
        result.sort();
        let expected_vector = expected
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        expected_vector.sort();
        assert_eq!(result, expected_vector);
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
