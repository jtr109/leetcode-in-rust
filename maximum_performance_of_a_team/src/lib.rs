pub struct Solution {}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 6;
        let speed = [2, 10, 3, 1, 5, 8];
        let efficiency = [5, 4, 3, 9, 7, 2];
        let k = 2;
        let expected = 60;
        assert_eq!(
            Solution::max_performance(n, speed.to_vec(), efficiency.to_vec(), k),
            expected
        );
    }

    #[test]
    fn example_2() {
        let n = 6;
        let speed = [2, 10, 3, 1, 5, 8];
        let efficiency = [5, 4, 3, 9, 7, 2];
        let k = 3;
        let expected = 68;
        assert_eq!(
            Solution::max_performance(n, speed.to_vec(), efficiency.to_vec(), k),
            expected
        );
    }

    #[test]
    fn example_3() {
        let n = 6;
        let speed = [2, 10, 3, 1, 5, 8];
        let efficiency = [5, 4, 3, 9, 7, 2];
        let k = 4;
        let expected = 72;
        assert_eq!(
            Solution::max_performance(n, speed.to_vec(), efficiency.to_vec(), k),
            expected
        );
    }
}
