#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let positive_reversed = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        if x.is_positive() {
            positive_reversed
        } else {
            -positive_reversed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::reverse(120), 21);
    }
}
