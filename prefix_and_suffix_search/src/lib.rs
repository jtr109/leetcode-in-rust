/*
 * [Explore - LeetCode](https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3728/)
 *
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

#[derive(Clone)]
struct Trie {
    children: Vec<Option<Box<Trie>>>,
    index: i32,
}

impl Trie {
    fn get_char_index(c: char) -> usize {
        if c as usize >= 'a' as usize && c as usize <= 'z' as usize {
            c as usize - 'a' as usize
        } else {
            26
        }
    }

    fn new() -> Self {
        Self {
            children: vec![None; 27],
            index: -1,
        }
    }

    fn add(&mut self, index: usize, chars: &str) {
        match chars.chars().nth(0) {
            None => (),
            Some(c) => {
                let c_index = Self::get_char_index(c);
                let child = self.children[c_index].get_or_insert(Box::new(Self::new()));
                child.add(index, &chars[1..]);
            }
        }
        self.index = index as i32; // mark index on the whole branch
    }

    fn search(&self, chars: &str) -> i32 {
        match chars.chars().nth(0) {
            None => self.index,
            Some(c) => {
                let c_index = Self::get_char_index(c);
                match &self.children[c_index] {
                    None => -1,
                    Some(child) => child.search(&chars[1..]),
                }
            }
        }
    }
}

pub struct WordFilter {
    trie: Trie,
}

impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for (index, word) in words.iter().enumerate() {
            for pattern in Self::all_patterns(word).iter() {
                trie.add(index, pattern);
            }
        }
        WordFilter { trie }
    }

    fn all_patterns(word: &str) -> Vec<String> {
        let mut res = vec![];
        for i in 0..word.len() {
            res.push(String::new() + &word[i..word.len()] + "#" + &word);
        }
        res
    }

    pub fn f(&self, prefix: String, suffix: String) -> i32 {
        self.trie.search(&(String::new() + &suffix + "#" + &prefix))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trie() {
        let mut root = Trie::new();
        root.add(0, "foo");
        root.add(1, "bar");
        root.add(3, "biz");
        assert_eq!(root.search("foo"), 0);
        assert_eq!(root.search("bar"), 1);
        assert_eq!(root.search("biz"), 3);
        assert_eq!(root.search("buz"), -1);
        assert_eq!(root.search("fo"), 0);
        assert_eq!(root.search("b"), 3);
    }

    #[test]
    fn example_1() {
        let words = vec!["apple"].iter().map(|s| s.to_string()).collect();
        let prefix = "a".to_string();
        let suffix = "e".to_string();
        let expected = 0;
        assert_eq!(WordFilter::new(words).f(prefix, suffix), expected);
    }

    #[test]
    fn submission_1() {
        let words = vec![
            "cabaabaaaa",
            "ccbcababac",
            "bacaabccba",
            "bcbbcbacaa",
            "abcaccbcaa",
            "accabaccaa",
            "cabcbbbcca",
            "ababccabcb",
            "caccbbcbab",
            "bccbacbcba",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
        let patterns = vec![
            vec!["bccbacbcba", "a"],
            vec!["ab", "abcaccbcaa"],
            vec!["a", "aa"],
            vec!["cabaaba", "abaaaa"],
            vec!["cacc", "accbbcbab"],
            vec!["ccbcab", "bac"],
            vec!["bac", "cba"],
            vec!["ac", "accabaccaa"],
            vec!["bcbb", "aa"],
            vec!["ccbca", "cbcababac"],
        ]
        .iter()
        .map(|p| vec![p[0].to_string(), p[1].to_string()])
        .collect::<Vec<Vec<String>>>();
        let result = patterns
            .iter()
            .map(|p| WordFilter::new(words.clone()).f(p[0].clone(), p[1].clone()))
            .collect::<Vec<i32>>();
        assert_eq!(result, vec![9, 4, 5, 0, 8, 1, 2, 5, 3, 1])
    }
}
