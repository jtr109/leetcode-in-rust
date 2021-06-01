pub struct Solution {}

#[derive(Clone)]
struct Trie {
    indices: Vec<usize>,
    children: Vec<Option<Box<Trie>>>,
}

impl Trie {
    fn new() -> Self {
        Self {
            indices: vec![],
            children: vec![None; 27],
        }
    }

    fn get_char_index(c: char) -> usize {
        if c as usize >= 'a' as usize && c as usize <= 'z' as usize {
            c as usize - 'a' as usize
        } else {
            26
        }
    }

    fn add(&mut self, index: usize, chars: &str) {
        match chars.chars().nth(0) {
            None => (),
            Some(c) => {
                let child =
                    self.children[Self::get_char_index(c)].get_or_insert(Box::new(Self::new()));
                child.add(index, &chars[1..]);
            }
        }
        self.indices.push(index);
    }

    fn search(&self, chars: &str) -> Vec<usize> {
        match chars.chars().nth(0) {
            None => self.indices.clone(),
            Some(c) => match &self.children[Self::get_char_index(c)] {
                None => vec![],
                Some(child) => child.search(&chars[1..]),
            },
        }
    }
}

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        // TODO: 深度优先搜索结果，但是怎么让它停下来？
        products.sort();
        let mut trie = Trie::new();
        for (index, p) in products.iter().enumerate() {
            trie.add(index, p);
        }
        let mut res = vec![];
        search_word.chars().fold(String::new(), |acc, x| {
            let chars = acc + &x.to_string();
            let indices = trie.search(&chars);
            res.push(
                indices
                    .iter()
                    .cloned()
                    .map(|i| products[i].clone())
                    .take(3)
                    .collect::<Vec<String>>(),
            );
            chars
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        let products = vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        for (i, p) in products.iter().enumerate() {
            trie.add(i, p);
        }
        assert_eq!(trie.search("m"), vec![0, 1, 2, 3, 4]);
        assert_eq!(trie.search("mo"), vec![0, 1, 2, 3, 4]);
        assert_eq!(trie.search("mou"), vec![1, 4]);
        assert_eq!(trie.search("mous"), vec![1, 4]);
        assert_eq!(trie.search("mouse"), vec![1, 4]);
    }

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
