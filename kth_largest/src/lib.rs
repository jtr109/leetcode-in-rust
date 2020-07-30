#![allow(dead_code)]

//! It works but slower than:
//! ```rust,no_run
//! impl Solution {
//!     pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
//!         let mut nums = nums;
//!         nums.sort_by(|a, b| b.cmp(a));
//!         nums[k as usize - 1]
//!     }
//! }
//! ```

struct Solution {}

struct Queue {
    k: usize,
    queue: Vec<i32>,
}

impl Queue {
    fn new(k: usize) -> Queue {
        Queue { k, queue: vec![] }
    }

    fn push(&mut self, n: i32) {
        let mut length = self.queue.len();
        if length < self.k {
            self.queue.push(n);
            length += 1;
        } else if self.queue[length - 1] < n {
            self.queue[length - 1] = n;
        } else {
            return;
        }
        // sort queue
        if length == 1 {
            return;
        }
        for i in (1..self.queue.len()).rev() {
            if self.queue[i - 1] < self.queue[i] {
                self.queue.swap(i - 1, i);
            } else {
                break;
            }
        }
    }

    fn kth(&self) -> i32 {
        self.queue[self.queue.len() - 1]
    }
}
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut queue = Queue::new(k as usize);
        for &n in nums.iter() {
            queue.push(n);
        }
        queue.kth()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let expected = 5;
        assert_eq!(expected, Solution::find_kth_largest(nums, k));
    }

    #[test]
    fn test_example2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let expected = 4;
        assert_eq!(expected, Solution::find_kth_largest(nums, k));
    }
}
