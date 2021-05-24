pub struct Solution {}

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' => (c as u8 - 'A' as u8 + 'a' as u8) as char,
                _ => c,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(s: &str, expected: &str) {
        assert_eq!(Solution::to_lower_case(s.to_string()), expected.to_string());
    }

    #[test]
    fn example_1() {
        ok("Hello", "hello");
    }

    #[test]
    fn example_2() {
        ok("here", "here");
    }

    #[test]
    fn example_3() {
        ok("LOVELY", "lovely");
    }
}
