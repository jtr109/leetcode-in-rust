pub struct Solution {}

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort();
        vertical_cuts.sort();
        let horizontal_lines = vec![vec![0], horizontal_cuts, vec![h]].concat();
        let vertical_lines = vec![vec![0], vertical_cuts, vec![w]].concat();
        (horizontal_lines
            .windows(2)
            .map(|w| w[1] - w[0])
            .max()
            .unwrap() as i64
            * vertical_lines
                .windows(2)
                .map(|w| w[1] - w[0])
                .max()
                .unwrap() as i64
            % (10_i64.pow(9) + 7)) as i32
    }
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
        let horizontal_cuts = [3];
        let vertical_cuts = [3];
        let expected = 9;
        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts.to_vec(), vertical_cuts.to_vec()),
            expected
        );
    }
}
