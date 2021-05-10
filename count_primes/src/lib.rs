/*!
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3738/
 */

pub struct Solution {}

impl Solution {
    // /// a normal way
    // pub fn count_primes(n: i32) -> i32 {
    //     (0..n)
    //         .filter(|x| *x == 2 || *x % 2 != 0)
    //         .filter(|x| Self::is_prime(x))
    //         .count() as i32
    // }

    // fn is_prime(x: &i32) -> bool {
    //     *x > 1 && !(2..*x).filter(|i| i * i <= *x).any(|i| *x % i == 0)
    // }

    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }

        let n = n as usize;
        let mut nums = vec![true; n];
        nums[0] = false;
        nums[1] = false;
        let mut i = 2;
        while i * i < n {
            if nums[i] {
                let mut j = 2;
                while i * j < n {
                    nums[i * j] = false;
                    j += 1;
                }
            }
            i += 1;
        }
        nums.iter().filter(|&&x| x).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_primes(10), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_primes(0), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::count_primes(1), 0);
    }
}
