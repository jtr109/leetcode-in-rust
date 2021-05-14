/*!
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3741/
 */

pub struct Solution {}

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut res = vec![];
        for i in 2..s.len() - 1 {
            let left = &s[1..i];
            let right = &s[i..s.len() - 1];
            for l in Self::create_possible(left).iter() {
                for r in Self::create_possible(right).iter() {
                    res.push(String::new() + "(" + l + ", " + r + ")");
                }
            }
        }
        res
    }

    fn create_possible(s: &str) -> Vec<String> {
        let mut possible = vec![];
        for i in 1..=s.len() {
            let left = &s[..i];
            let right = &s[i..];
            if (!left.starts_with("0") || left.len() == 1) && !right.ends_with("0") {
                possible
                    .push(String::new() + left + if right.len() != 0 { "." } else { "" } + right)
            }
        }
        possible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq(input: &str, output: &Vec<&str>) {
        let mut expected = output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        expected.sort();
        let mut result = Solution::ambiguous_coordinates(input.to_string());
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_1() {
        let input = "(123)";
        let output = vec!["(1, 23)", "(12, 3)", "(1.2, 3)", "(1, 2.3)"];
        assert_eq(input, &output);
    }

    #[test]
    fn example_2() {
        let input = "(00011)";
        let output = vec!["(0.001, 1)", "(0, 0.011)"];
        assert_eq(input, &output);
    }

    #[test]
    fn example_3() {
        let input = "(0123)";
        let output = vec![
            "(0, 123)",
            "(0, 12.3)",
            "(0, 1.23)",
            "(0.1, 23)",
            "(0.1, 2.3)",
            "(0.12, 3)",
        ];
        assert_eq(input, &output);
    }

    #[test]
    fn example_4() {
        let input = "(100)";
        let output = vec!["(10, 0)"];
        assert_eq(input, &output);
    }
}
