pub struct Solution {}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {}
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
                    .map(|x| x.to_vec())
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
        let expected = 0;
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
