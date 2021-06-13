use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        /*!
         * 解释可以查看[这个讨论](https://leetcode.com/problems/minimum-number-of-refueling-stops/discuss/294025/Java-Simple-Code-Greedy)。
         *
         * 主要思路：
         *
         * 把可以到达的最远距离中的所有站点的加油量加入优先队列备选，从中找到最大的加油量并弹出（避免重复加油），用以延长最远距离。
         * 最远距离延长后，可以选择的加油站也就增多了。将新增的可选加油站的加油量将钻石优先队列。
         *
         * 循环往复直到到达目标。
         *
         * 如果达到某个最远距离但是已经没有可以备选的加油站了，那么说明到不了终点。
         */
        let mut stops = 0;
        let mut furthest = start_fuel;
        let mut i = 0;
        let mut pq = BinaryHeap::new(); // 备选的加油量
        while furthest < target {
            while i < stations.len() && stations[i][0] <= furthest {
                pq.push(stations[i][1]);
                i += 1;
            }
            if pq.is_empty() {
                // 永远到不了 target
                return -1;
            }
            furthest += pq.pop().unwrap(); // 弹出备选的最大加油量，延长最远距离
            stops += 1;
        }
        stops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let target = 1;
        let start_fuel = 1;
        let stations = [];
        let expected = 0;
        assert_eq!(
            Solution::min_refuel_stops(
                target,
                start_fuel,
                stations
                    .to_vec()
                    .iter()
                    .map(|x: &[i32; 2]| x.to_vec())
                    .collect::<Vec<Vec<i32>>>()
            ),
            expected
        );
    }

    #[test]
    fn example_2() {
        let target = 100;
        let start_fuel = 1;
        let stations = [[10, 100]];
        let expected = -1;
        assert_eq!(
            Solution::min_refuel_stops(
                target,
                start_fuel,
                stations
                    .to_vec()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<i32>>>()
            ),
            expected
        );
    }

    #[test]
    fn example_3() {
        let target = 100;
        let start_fuel = 10;
        let stations = [[10, 60], [20, 30], [30, 30], [60, 40]];
        let expected = 2;
        assert_eq!(
            Solution::min_refuel_stops(
                target,
                start_fuel,
                stations
                    .to_vec()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<i32>>>()
            ),
            expected
        );
    }
}
