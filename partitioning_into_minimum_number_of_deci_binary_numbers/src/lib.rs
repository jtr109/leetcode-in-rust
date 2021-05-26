pub struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap() as i32 - '0' as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works(n: &str, expected: i32) {
        assert_eq!(Solution::min_partitions(n.to_string()), expected);
    }

    #[test]
    fn example_1() {
        it_works("32", 3);
    }

    #[test]
    fn example_2() {
        it_works("82734", 8);
    }

    #[test]
    fn example_3() {
        it_works("27346209830709182346", 9);
    }
}
