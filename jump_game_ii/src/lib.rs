pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut target = nums.len() - 1;
        let mut jumps = 0;
        'target_loop: loop {
            if target == 0 {
                break;
            }
            for cursor in 0..target {
                if cursor + nums[cursor] as usize >= target {
                    jumps += 1;
                    target = cursor;
                    continue 'target_loop;
                }
            }
        }
        jumps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }
}
