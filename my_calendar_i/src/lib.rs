struct MyCalendar {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {}

    fn book(&self, start: i32, end: i32) -> bool {}
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = MyCalendar::new();
        assert!(obj.book(10, 20));
        assert!(!obj.book(15, 25));
        assert!(obj.book(20, 30));
    }
}
