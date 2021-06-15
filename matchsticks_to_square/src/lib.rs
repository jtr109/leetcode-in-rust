pub struct Solution {}

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {}
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
