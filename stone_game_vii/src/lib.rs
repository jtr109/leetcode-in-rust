pub struct Solution {}

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let stones = [5, 3, 1, 4, 2];
        let expected = 6;
        assert_eq!(Solution::stone_game_vii(stones.to_vec()), expected);
    }

    #[test]
    fn example_2() {
        let stones = [7, 90, 5, 1, 100, 10, 10, 2];
        let expected = 122;
        assert_eq!(Solution::stone_game_vii(stones.to_vec()), expected);
    }
}
