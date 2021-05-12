pub struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    cache: Vec<Vec<Option<i32>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self {
            cache: vec![vec![None; matrix[0].len()]; matrix.len()],
            matrix,
        }
    }

    fn sum(&mut self, row: i32, col: i32) -> i32 {
        if row == -1 || col == -1 {
            return 0;
        }
        if let Some(s) = self.cache[row as usize][col as usize] {
            return s;
        }
        let s = self.matrix[row as usize][col as usize]
            + self.sum(row - 1, col)
            + self.sum(row, col - 1)
            - self.sum(row - 1, col - 1);
        self.cache[row as usize][col as usize] = Some(s);
        s
    }

    pub fn sum_region(&mut self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sum(row2, col2) - self.sum(row1 - 1, col2) - self.sum(row2, col1 - 1)
            + self.sum(row1 - 1, col1 - 1)
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let mut obj = NumMatrix::new(matrix);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }
}
