pub struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                max = max.max(Self::cal_area(&mut grid, i as i32, j as i32));
            }
        }
        max
    }

    fn cal_area(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let mut area = 0;
        if grid[i as usize][j as usize] == 0 {
            return area;
        }
        grid[i as usize][j as usize] = 0;
        area += 1;
        for (r, c) in vec![(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            if r < 0 || r == grid.len() as i32 || c < 0 || c == grid[0].len() as i32 {
                continue;
            }
            area += Self::cal_area(grid, r, c);
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_area() {
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
        let mut grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(Solution::cal_area(&mut grid, 0, 2), 1);
        assert_eq!(Solution::cal_area(&mut grid, 0, 7), 4);
        assert_eq!(Solution::cal_area(&mut grid, 1, 7), 0);
    }

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
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        let expected = 6;
        assert_eq!(Solution::max_area_of_island(grid), expected);
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
