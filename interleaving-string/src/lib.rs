pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {}
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
}
