pub struct Solution {}

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let target = sum / 4;
        let sums = vec![0; 4];
        let nums = matchsticks.iter().cloned().rev().collect();
        Self::dfs(&nums, &sums, 0, target)
    }

    fn dfs(nums: &Vec<i32>, sums: &Vec<i32>, index: usize, target: i32) -> bool {
        if index == nums.len() {
            return sums.iter().all(|x| *x == target);
        }
        let n = nums[index];
        for (i, s) in sums.iter().enumerate() {
            if s + n > target {
                continue;
            }
            let mut new_sums = sums.clone();
            new_sums[i] = s + n;
            if Self::dfs(nums, &new_sums, index + 1, target) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let matchsticks = [1, 1, 2, 2, 2];
        assert!(Solution::makesquare(matchsticks.to_vec()));
    }

    #[test]
    fn example_2() {
        let matchsticks = [3, 3, 3, 3, 4];
        assert!(!Solution::makesquare(matchsticks.to_vec()));
    }
}
