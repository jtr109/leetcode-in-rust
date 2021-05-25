const OPERATORS: [&str; 4] = ["+", "-", "*", "/"];

pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums: Vec<i32> = vec![];
        for t in tokens.iter() {
            if !OPERATORS.iter().any(|x| x == t) {
                nums.push(t.parse().unwrap());
                continue;
            }
            let n2 = nums.pop().unwrap();
            let n1 = nums.pop().unwrap();
            nums.push(if t == "+" {
                n1 + n2
            } else if t == "-" {
                n1 - n2
            } else if t == "*" {
                n1 * n2
            } else {
                n1 / n2
            })
        }
        nums.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(tokens: Vec<&str>, expected: i32) {
        let tokens: Vec<String> = tokens.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::eval_rpn(tokens), expected);
    }

    #[test]
    fn example_1() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        let expected = 9;
        ok(tokens, expected);
    }

    #[test]
    fn example_2() {
        let tokens = vec!["4", "13", "5", "/", "+"];
        let expected = 6;
        ok(tokens, expected);
    }

    #[test]
    fn example_3() {
        let tokens = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let expected = 22;
        ok(tokens, expected);
    }
}
