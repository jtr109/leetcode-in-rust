pub struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.matrix[row1 as usize..=row2 as usize]
            .iter()
            .map(|row| row[col1 as usize..=col2 as usize].iter().sum::<i32>())
            .sum::<i32>()
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
        let obj = NumMatrix::new(matrix);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }
}
