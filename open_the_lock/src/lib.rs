pub struct Solution {}

impl Solution {
    fn digit_neighbors(c: char) -> Vec<char> {
        let n = c.to_digit(10).unwrap();
        vec![
            char::from_digit((n + 9) % 10, 10).unwrap(),
            char::from_digit((n + 1) % 10, 10).unwrap(),
        ]
    }

    fn neighbors(wheels: &str) -> Vec<String> {
        wheels
            .char_indices()
            .map(|(i, c)| {
                Solution::digit_neighbors(c)
                    .iter()
                    .map(|x| {
                        let mut w = wheels.to_string();
                        w.replace_range(i..i + 1, &x.to_string());
                        w
                    })
                    .collect()
            })
            .collect::<Vec<Vec<String>>>()
            .concat()
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut marked = vec![false; 10000];
        let mut queue = vec![];
        let mut depth = 0;
        queue.push("0000".to_string());
        loop {
            let mut new_queue = vec![];
            for current in queue {
                if current == target {
                    return depth;
                }
                for neighbor in Self::neighbors(&current) {
                    let index: usize = neighbor.parse().unwrap();
                    if !marked[index] {
                        marked[index] = true;
                        new_queue.push(neighbor);
                    }
                }
            }
            if new_queue.is_empty() {
                return -1;
            }
            queue = new_queue;
            depth += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_neighbors() {
        assert_eq!(Solution::digit_neighbors('0'), vec!['9', '1']);
        assert_eq!(Solution::digit_neighbors('9'), vec!['8', '0']);
        for n in 1..9 {
            assert_eq!(
                Solution::digit_neighbors(char::from_digit(n, 10).unwrap()),
                vec![
                    char::from_digit(n - 1, 10).unwrap(),
                    char::from_digit(n + 1, 10).unwrap()
                ]
            );
        }
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            Solution::neighbors("0201"),
            vec!["9201", "1201", "0101", "0301", "0291", "0211", "0200", "0202"],
        );
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
