pub struct Solution {}

impl Solution {
    fn max_res(nums: &Vec<i32>, i: usize, k: usize) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        nums[i]
            + (1..=k)
                .map(|x| Self::max_res(nums, i + x, k))
                .max()
                .unwrap()
    }

    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_res(&nums, 0, k as usize)
    }
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
