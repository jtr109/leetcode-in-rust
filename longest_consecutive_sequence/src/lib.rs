pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut numbers = nums.clone();
        numbers.sort();
        numbers.dedup();

        let mut max_length = 0;
        let mut temp_length = 1;
        for n in numbers.windows(2) {
            let j = n[1];
            let i = n[0];
            if j == i + 1 {
                temp_length += 1;
                continue;
            }
            max_length = max_length.max(temp_length);
            temp_length = 1;
        }
        max_length = max_length.max(temp_length);
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [100, 4, 200, 1, 3, 2];
        let expected = 4;
        assert_eq!(Solution::longest_consecutive(nums.to_vec()), expected);
    }

    #[test]
    fn example_2() {
        let nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expected = 9;
        assert_eq!(Solution::longest_consecutive(nums.to_vec()), expected);
    }
}
