pub struct Solution {}

struct Resolver {
    v1: Vec<char>,
    v2: Vec<char>,
    v3: Vec<char>,
    validation: Vec<Vec<Option<bool>>>,
}

impl Resolver {
    fn new(s1: String, s2: String, s3: String) -> Self {
        Self {
            v1: s1.chars().collect(),
            v2: s2.chars().collect(),
            v3: s3.chars().collect(),
            validation: vec![vec![None; s2.chars().count()]; s1.chars().count()],
        }
    }

    fn dfs(&self, i1: usize, i2: usize, i3: usize) -> bool {
        if i3 == self.v3.len() {
            return true;
        }
        i1 < self.v1.len() && self.v1[i1] == self.v3[i3] && self.dfs(i1 + 1, i2, i3 + 1)
            || i2 < self.v2.len() && self.v2[i2] == self.v3[i3] && self.dfs(i1, i2 + 1, i3 + 1)
    }

    fn is_interleave(&self) -> bool {
        if self.v1.len() + self.v2.len() != self.v3.len() {
            return false;
        }
        self.dfs(0, 0, 0)
    }
}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Resolver::new(s1, s2, s3).is_interleave()
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
