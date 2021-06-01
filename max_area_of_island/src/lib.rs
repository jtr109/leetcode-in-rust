pub struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let expected = 6;
        assert_eq!(
            Solution::max_area_of_island(grid.iter().map(|row| row.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn example_2() {
        let grid = [[0, 0, 0, 0, 0, 0, 0, 0]];
        let expected = 0;
        assert_eq!(
            Solution::max_area_of_island(grid.iter().map(|row| row.to_vec()).collect()),
            expected
        );
    }
}
