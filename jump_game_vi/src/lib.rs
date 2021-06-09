pub struct Solution {}

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, -1, -2, 4, -7, 3];
        let k = 2;
        let expected = 7;
        assert_eq!(Solution::max_result(nums.to_vec(), k), expected);
    }

    #[test]
    fn example_2() {
        let nums = [10, -5, -2, 4, 0, 3];
        let k = 3;
        let expected = 17;
        assert_eq!(Solution::max_result(nums.to_vec(), k), expected);
    }

    #[test]
    fn example_3() {
        let nums = [1, -5, -20, 4, -1, 3, -6, -3];
        let k = 2;
        let expected = 0;
        assert_eq!(Solution::max_result(nums.to_vec(), k), expected);
    }
}
