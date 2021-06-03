pub struct Solution {}

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = [1, 2, 4];
        let vertical_cuts = [1, 3];
        let expected = 4;
        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts.to_vec(), vertical_cuts.to_vec()),
            expected
        );
    }

    #[test]
    fn example_2() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = [3, 1];
        let vertical_cuts = [1];
        let expected = 6;
        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts.to_vec(), vertical_cuts.to_vec()),
            expected
        );
    }

    #[test]
    fn example_3() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = [3, 1];
        let vertical_cuts = [3];
        let expected = 9;
        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts.to_vec(), vertical_cuts.to_vec()),
            expected
        );
    }
}
