pub struct Solution {}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works(nums: Vec<i32>, expected: i32) {
        assert_eq!(Solution::maximum_gap(nums), expected);
    }

    #[test]
    fn example_1() {
        let nums = vec![3, 6, 9, 1];
        let expected = 3;
        it_works(nums, expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![10];
        let expected = 0;
        it_works(nums, expected);
    }
}
