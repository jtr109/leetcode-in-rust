pub struct Solution {}

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut count = 0;
        let mut at_most_left = 0; // 从左往右最后一个大于等于 left 的元素对应的索引
        let mut subarray_left = 0;
        for i in 0..nums.len() {
            if nums[i] > right {
                count += at_most_left - subarray_left;
                at_most_left = i + 1;
                subarray_left = i + 1;
                continue;
            }
            count += 1;
            if nums[i] >= left {
                at_most_left = i;
            }
        }
        count as i32
    }
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

    #[test]
    fn submission_1() {
        let nums = [2, 9, 2, 5, 6];
        let left = 2;
        let right = 8;
        let expected = 7;
        assert_eq!(
            Solution::num_subarray_bounded_max(nums.to_vec(), left, right),
            expected
        );
    }
}
