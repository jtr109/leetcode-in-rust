pub struct Solution {}

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut max = nums[l];
        loop {
            r += 1;
            if r == nums.len() {
                break;
            }
            if nums[l..r].contains(&nums[r]) {
                while nums[l] != nums[r] {
                    l += 1;
                }
                l += 1;
            }
            let sum = nums[l..=r].iter().sum();
            if max < sum {
                max = sum;
            }
        }
        max
    }
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

    #[test]
    fn submission_test_1() {
        let nums = vec![1000];
        let expected = 1000;
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
    }
}
