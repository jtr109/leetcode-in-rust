/*!
 * Follow solution: https://leetcode.com/problems/generate-parentheses/discuss/10100/Easy-to-understand-Java-backtracking-solution
 */

pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut list = Vec::new();
        Self::backtrack(&mut list, "".to_string(), 0, 0, n as usize);
        list
    }

    fn backtrack(list: &mut Vec<String>, s: String, open: usize, close: usize, max: usize) {
        // 如果前缀为 s 的值，获取所有可能性，加入 list
        if (s.chars().count()) == max * 2 {
            list.push(s);
            return;
        }

        if open < max {
            // 如果前缀中的 `(` 数量达到最大值，不再往前缀中加入 `(`
            Self::backtrack(list, s.clone() + "(", open + 1, close, max);
        }
        if close < open {
            // 如果前缀中的 `)` 数量和 `(` 数量一样多了，不再往前缀中加入 `)`
            Self::backtrack(list, s.clone() + ")", open, close + 1, max);
        }
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
