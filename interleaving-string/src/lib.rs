pub struct Solution {}

impl Solution {
    fn interleave(
        s1: &Vec<char>,
        s2: &Vec<char>,
        s3: &Vec<char>,
        i1: usize,
        i2: usize,
        i3: usize,
    ) -> bool {
        if i3 == s3.len() {
            return true;
        }
        i1 < s1.len() && s1[i1] == s3[i3] && Self::interleave(s1, s2, s3, i1 + 1, i2, i3 + 1)
            || i2 < s2.len() && s2[i2] == s3[i3] && Self::interleave(s1, s2, s3, i1, i2 + 1, i3 + 1)
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Self::interleave(
            &s1.chars().collect::<Vec<char>>(),
            &s2.chars().collect::<Vec<char>>(),
            &s3.chars().collect::<Vec<char>>(),
            0,
            0,
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_expected(s1: &str, s2: &str, s3: &str, expected: bool) {
        assert_eq!(
            Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string()),
            expected
        );
    }

    #[test]
    fn example_1() {
        let s1 = "aabcc";
        let s2 = "dbbca";
        let s3 = "aadbbcbcac";
        let expected = true;
        as_expected(s1, s2, s3, expected);
    }

    #[test]
    fn example_2() {
        let s1 = "aabcc";
        let s2 = "dbbca";
        let s3 = "aadbbbaccc";
        let expected = false;
        as_expected(s1, s2, s3, expected);
    }

    #[test]
    fn example_3() {
        let s1 = "";
        let s2 = "";
        let s3 = "";
        let expected = true;
        as_expected(s1, s2, s3, expected);
    }

    #[test]
    fn submission_1() {
        let s1 = "bbbbbabbbbabaababaaaabbababbaaabbabbaaabaaaaababbbababbbbbabbbbababbabaabababbbaabababababbbaaababaa";
        let s2 = "babaaaabbababbbabbbbaabaabbaabbbbaabaaabaababaaaabaaabbaaabaaaabaabaabbbbbbbbbbbabaaabbababbabbabaab";
        let s3 = "babbbabbbaaabbababbbbababaabbabaabaaabbbbabbbaaabbbaaaaabbbbaabbaaabababbaaaaaabababbababaababbababbbababbbbaaaabaabbabbaaaaabbabbaaaabbbaabaaabaababaababbaaabbbbbabbbbaabbabaabbbbabaaabbababbabbabbab";
        let expected = true;
        as_expected(s1, s2, s3, expected);
    }
}
