pub struct Solution {}

impl Solution {
    fn min_cost_to_step(cost: &Vec<i32>, i: usize) -> i32 {
        if i < 2 {
            return 0;
        }
        (Self::min_cost_to_step(cost, i - 1) + cost[i - 1])
            .min(Self::min_cost_to_step(cost, i - 2) + cost[i - 2])
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        Self::min_cost_to_step(&cost, cost.len())
    }
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
