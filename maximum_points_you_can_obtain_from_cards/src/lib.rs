pub struct Solution {}

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let window_length = card_points.len() - k;
        let mut current: i32 = card_points[window_length..].iter().sum();
        let max = current;
        (0..k).fold(max, |acc, i| {
            current += card_points[i] - card_points[i + window_length];
            acc.max(current)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;
        let expected = 12;
        assert_eq!(Solution::max_score(card_points, k), expected);
    }

    #[test]
    fn example_2() {
        let card_points = vec![2, 2, 2];
        let k = 2;
        let expected = 4;
        assert_eq!(Solution::max_score(card_points, k), expected);
    }

    #[test]
    fn example_3() {
        let card_points = vec![9, 7, 7, 9, 7, 7, 9];
        let k = 7;
        let expected = 55;
        assert_eq!(Solution::max_score(card_points, k), expected);
    }

    #[test]
    fn example_4() {
        let card_points = vec![1, 1000, 1];
        let k = 1;
        let expected = 1;
        assert_eq!(Solution::max_score(card_points, k), expected);
    }

    #[test]
    fn example_5() {
        let card_points = vec![1, 79, 80, 1, 1, 1, 200, 1];
        let k = 3;
        let expected = 202;
        assert_eq!(Solution::max_score(card_points, k), expected);
    }
}
