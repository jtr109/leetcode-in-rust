pub struct MyCalendar {
    books: Vec<(i32, i32)>,
}

impl MyCalendar {
    pub fn new() -> Self {
        Self { books: vec![] }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        let mut i = 0;
        for (j, (s, e)) in self.books.iter().enumerate() {
            if *e <= start {
                continue;
            };
            if end > *s {
                return false;
            }
            i = j;
        }
        self.books.insert(i, (start, end));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MyCalendar::new();
        assert!(obj.book(10, 20));
        assert!(!obj.book(15, 25));
        assert!(obj.book(20, 30));
    }
}
