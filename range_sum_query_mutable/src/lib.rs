struct NumArray {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {}

    fn update(&self, index: i32, val: i32) {}

    fn sum_range(&self, left: i32, right: i32) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 3, 5];
        let obj = NumArray::new(nums.to_vec());
        assert_eq!(obj.sum_range(0, 2), 9);
        obj.update(1, 2);
        assert_eq!(obj.sum_range(0, 2), 8);
    }
}
