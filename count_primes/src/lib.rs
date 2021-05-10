/*!
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3738/
 */

pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        (0..n)
            // .filter(|x| *x == 2 || *x % 2 != 0)
            .filter(|x| Self::is_prime(x))
            .count() as i32
    }

    pub fn is_prime(x: &i32) -> bool {
        *x > 1 && !(2..*x).any(|i| *x % i == 0)
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
