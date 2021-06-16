pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        }
        let mut result = Vec::new();
        for r in Self::generate_parenthesis(n - 1) {
            result.push(format!("({})", r));
            let s1 = format!("(){}", r);
            let s2 = format!("{}()", r);
            if s2 != s1 {
                result.push(s2);
            }
            result.push(s1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works(n: i32, expected: Vec<&str>) {
        let mut result = Solution::generate_parenthesis(n);
        let mut exp = expected.to_vec();
        result.sort();
        exp.sort();
        assert_eq!(result, exp);
    }

    #[test]
    fn example_1() {
        let n = 3;
        let expected = ["((()))", "(()())", "(())()", "()(())", "()()()"];
        it_works(n, expected.to_vec());
    }

    #[test]
    fn example_2() {
        let n = 1;
        let expected = ["()"];
        it_works(n, expected.to_vec());
    }

    #[test]
    fn submission_1() {
        let n = 4;
        let expected = [
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ];
        it_works(n, expected.to_vec());
    }
}
