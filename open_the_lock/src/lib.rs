pub struct Solution {}

impl Solution {
    fn digit_neighbors(n: u8) -> (u8, u8) {
        ((n + 9) % 10, (n + 11) % 10)
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_neighbors() {
        assert_eq!(Solution::digit_neighbors(0), (9, 1));
        assert_eq!(Solution::digit_neighbors(9), (8, 0));
        for n in 1..9 {
            assert_eq!(Solution::digit_neighbors(n), (n - 1, n + 1));
        }
    }

    #[test]
    fn example_1() {
        let deadends = ["0201", "0101", "0102", "1212", "2002"];
        let target = "0202";
        let expected = 6;
        assert_eq!(
            Solution::open_lock(
                deadends.iter().map(|x| x.to_string()).collect(),
                target.to_string(),
            ),
            expected
        );
    }

    #[test]
    fn example_2() {
        let deadends = ["8888"];
        let target = "0009";
        let expected = 1;
        assert_eq!(
            Solution::open_lock(
                deadends.iter().map(|x| x.to_string()).collect(),
                target.to_string(),
            ),
            expected
        );
    }

    #[test]
    fn example_3() {
        let deadends = [
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ];
        let target = "8888";
        let expected = -1;
        assert_eq!(
            Solution::open_lock(
                deadends.iter().map(|x| x.to_string()).collect(),
                target.to_string(),
            ),
            expected
        );
    }

    #[test]
    fn example_4() {
        let deadends = ["0000"];
        let target = "8888";
        let expected = -1;
        assert_eq!(
            Solution::open_lock(
                deadends.iter().map(|x| x.to_string()).collect(),
                target.to_string(),
            ),
            expected
        );
    }
}
