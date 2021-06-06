pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [100, 4, 200, 1, 3, 2];
        let expected = 4;
        assert_eq!(Solution::longest_consecutive(nums.to_vec()), expected);
    }

    fn example_2() {
        let nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expected = 9;
        assert_eq!(Solution::longest_consecutive(nums.to_vec()), expected);
    }
}
