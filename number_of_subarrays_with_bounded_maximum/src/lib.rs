pub struct Solution {}

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [2, 1, 4, 3];
        let left = 2;
        let right = 3;
        let expected = 3;
        assert_eq!(
            Solution::num_subarray_bounded_max(nums.to_vec(), left, right),
            expected
        );
    }
}
