pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 3;
        let expected = ["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parenthesis(n), expected.to_vec());
    }

    #[test]
    fn example_2() {
        let n = 1;
        let expected = ["()"];
        assert_eq!(Solution::generate_parenthesis(n), expected.to_vec());
    }
}
