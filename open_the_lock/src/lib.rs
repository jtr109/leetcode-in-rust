pub struct Solution {}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
