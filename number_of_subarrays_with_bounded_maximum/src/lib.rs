pub struct Solution {}

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut count = 0;
        let mut left_boundary = 0;
        let mut at_most_left = 0; // 从左往右最后一个大于等于 left 的元素对应的索引
        for right_boundary in 0..nums.len() {
            if nums[right_boundary] > right {
                // subarray 中出现大于 right 的数字，销毁
                at_most_left = right_boundary as i32 + 1;
                left_boundary = right_boundary as i32 + 1;
            } else if nums[right_boundary] < left {
                // 从 left_boundary 到 at_most_left 之间移动左边界，形成的子集都符合要求。
                count += at_most_left - left_boundary + 1;
            } else {
                at_most_left = right_boundary as i32;
                count += at_most_left - left_boundary + 1;
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

    #[test]
    fn submission_2() {
        let nums = [16, 69, 88, 85, 79, 87, 37, 33, 39, 34];
        let left = 55;
        let right = 57;
        let expected = 0;
        assert_eq!(
            Solution::num_subarray_bounded_max(nums.to_vec(), left, right),
            expected
        );
    }
}
