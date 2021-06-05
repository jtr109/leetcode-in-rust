use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut groups = speed
            .into_iter()
            .zip(efficiency.into_iter())
            .collect::<Vec<(i32, i32)>>();
        groups.sort_by(|a, b| b.1.cmp(&a.1));
        let (speeds, efficiencies): (Vec<i32>, Vec<i32>) = groups.iter().cloned().unzip();
        let mut speed_queue = BinaryHeap::new(); // lowest priority queue
        let mut total_speed = 0;
        for i in 0..k as usize {
            speed_queue.push(Reverse(speeds[i]));
            total_speed += speeds[i];
        }
        let mut result = total_speed * efficiencies[k as usize - 1];
        // try another new engineer with lower efficiency
        for i in k as usize..n as usize {
            let lowest_speed = speed_queue.peek().unwrap().0;
            let new_speed = speeds[i];
            if lowest_speed > new_speed {
                continue;
            }
            let new_result = (total_speed + new_speed - lowest_speed) * efficiencies[i];
            if new_result < result {
                continue;
            }
            speed_queue.pop();
            speed_queue.push(Reverse(new_speed));
            result = new_result;
        }
        result
    }
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

    #[test]
    fn submission_1() {
        let n = 3;
        let speed = [2, 8, 2];
        let efficiency = [2, 7, 1];
        let k = 2;
        let expected = 56;
        assert_eq!(
            Solution::max_performance(n, speed.to_vec(), efficiency.to_vec(), k),
            expected
        );
    }
}
