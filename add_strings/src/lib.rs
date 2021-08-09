pub struct Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let max_length = num1.len().max(num2.len());
        let mut reversed_digits1 = num1
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        let mut reversed_digits2 = num2
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        while reversed_digits1.len() < max_length {
            reversed_digits1.push(0);
        }
        while reversed_digits2.len() < max_length {
            reversed_digits2.push(0);
        }
        let mut reversed_res = vec![];
        let mut digit = 0;
        for i in 0..max_length {
            let sum = digit + reversed_digits1[i] + reversed_digits2[i];
            reversed_res.push(sum % 10);
            digit = sum / 10;
        }
        if digit != 0 {
            reversed_res.push(digit);
        }
        reversed_res.iter().rev().map(|d| d.to_string()).collect()
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
