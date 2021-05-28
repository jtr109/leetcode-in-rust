pub struct Solution {}

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![4, 2, 4, 5, 6];
        let expected = 17;
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let expected = 8;
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
    }
}
