pub struct Solution {}

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let index = nums.len() / 2;
        let target = nums[index];
        let left_count = nums[..index].iter().count();
        let left_sum: i32 = nums[..index].iter().sum();
        let right_count = nums[index..].iter().count();
        let right_sum: i32 = nums[index..].iter().sum();
        target * left_count as i32 - left_sum + right_sum - target * right_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3];
        let expected = 2;
        assert_eq!(Solution::min_moves2(nums), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 10, 2, 9];
        let expected = 16;
        assert_eq!(Solution::min_moves2(nums), expected);
    }

    #[test]
    fn boundary() {
        let nums = vec![1];
        let expected = 0;
        assert_eq!(Solution::min_moves2(nums), expected);
    }
}
