use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        for (l, n) in nums.iter().enumerate() {
            let mut subarray_nums = HashSet::new();
            subarray_nums.insert(*n);
            for r in l + 1..nums.len() {
                let m = nums[r];
                if subarray_nums.contains(&m) {
                    break;
                }
                subarray_nums.insert(m);
            }
            let sum = subarray_nums.iter().sum();
            if sum > max {
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
}
