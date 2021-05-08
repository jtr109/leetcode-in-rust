#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let common_length = Self::lcs(&word1, &word2, word1.len(), word2.len());
        (word1.len() + word2.len() - 2 * common_length) as i32
    }

    pub fn lcs(s1: &str, s2: &str, m: usize, n: usize) -> usize {
        // comparing first m characters in s1 and first n characters in s2
        if m == 0 || n == 0 {
            0
        } else if s1.chars().nth(m - 1) == s2.chars().nth(n - 1) {
            1 + Self::lcs(s1, s2, m - 1, n - 1)
        } else {
            Self::lcs(s1, s2, m - 1, n).max(Self::lcs(s1, s2, m, n - 1))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1_lcs() {
        let word1 = "sea";
        let word2 = "eat";
        let expected = 2;
        assert_eq!(
            Solution::lcs(word1, word2, word1.len(), word2.len()),
            expected
        );
    }

    #[test]
    fn example_1() {
        let word1 = "sea";
        let word2 = "eat";
        let expected = 2;
        assert_eq!(
            Solution::min_distance(word1.to_string(), word2.to_string()),
            expected
        );
    }
}
