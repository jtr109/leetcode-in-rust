pub struct Solution {}

enum Turn {
    AliceTurn,
    BobTurn,
}

impl Solution {
    fn operate(stones: &Vec<i32>, i: usize, j: usize, turn: &Turn) -> i32 {
        if i + 1 == j {
            return 0;
        }
        let next_turn = match turn {
            Turn::AliceTurn => Turn::BobTurn,
            Turn::BobTurn => Turn::AliceTurn,
        };
        let stones_without_leftmost = &stones[i + 1..j];
        let difference_removing_leftmost = Self::operate(stones, i + 1, j, &next_turn); // 如果移动最左边的石头，后续产生的差异
        let score_removing_leftmost = stones_without_leftmost.iter().sum::<i32>();
        let stones_without_rightmost = &stones[i..j - 1];
        let difference_removing_rightmost = Self::operate(stones, i, j - 1, &next_turn);
        let score_removing_rightmost = stones_without_rightmost.iter().sum::<i32>();
        match turn {
            Turn::AliceTurn => (difference_removing_leftmost + score_removing_leftmost)
                .max(difference_removing_rightmost + score_removing_rightmost),
            Turn::BobTurn => (difference_removing_leftmost - score_removing_leftmost)
                .min(difference_removing_rightmost - score_removing_rightmost),
        }
    }

    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        Self::operate(&stones, 0, stones.len(), &Turn::AliceTurn)
    }
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
