struct Solution {}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works(products: Vec<&str>, word: &str, expected: Vec<Vec<&str>>) {
        assert_eq!(
            Solution::suggested_products(
                products.iter().map(|x| x.to_string()).collect(),
                word.to_string()
            ),
            expected
                .iter()
                .map(|x| x.iter().map(|y| y.to_string()).collect())
                .collect::<Vec<Vec<String>>>()
        )
    }

    #[test]
    fn example_1() {
        let products = vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        let word = "mouse";
        let expected = vec![
            vec!["mobile", "moneypot", "monitor"],
            vec!["mobile", "moneypot", "monitor"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
        ];
        it_works(products, word, expected);
    }

    #[test]
    fn example_2() {
        let products = vec!["havana"];
        let word = "havana";
        let expected = vec![
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
        ];
        it_works(products, word, expected);
    }

    #[test]
    fn example_3() {
        let products = vec!["bags", "baggage", "banner", "box", "cloths"];
        let word = "bags";
        let expected = vec![
            vec!["baggage", "bags", "banner"],
            vec!["baggage", "bags", "banner"],
            vec!["baggage", "bags"],
            vec!["bags"],
        ];
        it_works(products, word, expected);
    }
}
