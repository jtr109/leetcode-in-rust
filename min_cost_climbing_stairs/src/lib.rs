pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let cost = [10, 15, 20];
        let expected = 15;
        assert_eq!(Solution::min_cost_climbing_stairs(cost.to_vec()), expected);
    }

    #[test]
    fn example_2() {
        let cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let expected = 6;
        assert_eq!(Solution::min_cost_climbing_stairs(cost.to_vec()), expected);
    }
}
