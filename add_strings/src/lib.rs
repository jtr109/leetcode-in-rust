pub struct Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let max_length = num1.len().max(num2.len());
        let mut digits1 = num1
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        let mut digits2 = num2
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        while digits1.len() < max_length {
            digits1.push(0);
        }
        while digits2.len() < max_length {
            digits2.push(0);
        }
        (0..max_length)
            .fold(0, |acc, i| {
                acc * 10 + digits1[max_length - i - 1] + digits2[max_length - i - 1]
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_ok(num1: &str, num2: &str, expected: &str) {
        assert_eq!(
            Solution::add_strings(num1.to_string(), num2.to_string()),
            expected
        );
    }

    #[test]
    fn test_example1() {
        let num1 = "11";
        let num2 = "123";
        let expected = "134";
        is_ok(num1, num2, expected);
    }

    #[test]
    fn test_example2() {
        let num1 = "456";
        let num2 = "77";
        let expected = "533";
        is_ok(num1, num2, expected);
    }

    #[test]
    fn test_example3() {
        let num1 = "0";
        let num2 = "0";
        let expected = "0";
        is_ok(num1, num2, expected);
    }

    #[test]
    fn test_submission1() {
        let num1 = "6913259244";
        let num2 = "71103343";
        let expected = "6984362587";
        is_ok(num1, num2, expected);
    }
}
