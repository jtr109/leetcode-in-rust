pub struct Solution {}

impl Solution {
    fn interleave(s1: &[char], s2: &[char], s3: &[char]) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }
        if s3.len() == 0 {
            return s1.len() == 0 && s2.len() == 0;
        }
        if s1.len() == 0 {
            s2 == s3
        } else if s2.len() == 0 {
            s1 == s3
        } else if s1[0] == s3[0] && Self::interleave(&s1[1..].to_vec(), s2, &s3[1..].to_vec()) {
            true
        } else if s2[0] == s3[0] && Self::interleave(s1, &s2[1..], &s3[1..]) {
            true
        } else {
            false
        }
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Self::interleave(
            &s1.chars().collect::<Vec<char>>()[..],
            &s2.chars().collect::<Vec<char>>()[..],
            &s3.chars().collect::<Vec<char>>()[..],
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
