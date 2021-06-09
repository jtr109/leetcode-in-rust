pub struct Solution {}

impl Solution {
    fn max_res(nums: &Vec<i32>, i: usize, k: usize, cache: &mut Vec<Option<i32>>) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        if let Some(res) = cache[i] {
            return res;
        }
        let res = nums[i]
            + (1..=k)
                .map(|x| Self::max_res(nums, i + x, k, cache))
                .max()
                .unwrap();
        cache[i] = Some(res);
        res
    }

    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut cache = vec![None; nums.len() + 2];
        Self::max_res(&nums, 0, k as usize, &mut cache)
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
