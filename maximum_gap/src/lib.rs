pub struct Solution {}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        /*!
        使用 bucket sort 实现
        思路：控制每个 bucket 的范围小于最小值和最大值之间元素数量不变，但是等间隔时的间隔范围
        判断 buckets 之间前一个 bucket 的最大值和后一个 bucket 的最小值之差，取结果中的最大值
         */
        if nums.len() < 2 {
            return 0;
        }

        let lo = nums.iter().min().unwrap();
        let hi = nums.iter().max().unwrap();
        if lo == hi {
            return 0;
        }

        let mut buckets = vec![vec![]; nums.len() + 1];
        for n in nums.iter() {
            let index = (n - lo) as usize * (nums.len() - 1) / (hi - lo) as usize;
            buckets[index].push(*n);
        }
        let pairs = buckets
            .iter()
            .filter(|b| b.len() > 0)
            .map(|b| (*b.iter().min().unwrap(), *b.iter().max().unwrap()))
            .collect::<Vec<(i32, i32)>>();
        let mut max_gap = 0;
        for i in 0..pairs.len() - 1 {
            max_gap = max_gap.max(pairs[i + 1].0 - pairs[i].1);
        }
        max_gap
    }
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

    #[test]
    fn submission_test_1() {
        let nums = vec![1, 10000000];
        let expected = 9999999;
        it_works(nums, expected);
    }

    #[test]
    fn submission_test_2() {
        let nums = vec![1, 1, 1, 1];
        let expected = 0;
        it_works(nums, expected);
    }

    #[test]
    fn submission_test_3() {
        let nums = vec![1, 1, 1, 1, 1, 5, 5, 5, 5, 5];
        let expected = 4;
        it_works(nums, expected);
    }
}
