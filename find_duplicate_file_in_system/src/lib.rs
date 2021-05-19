use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn parse_path(path: &str) -> Vec<(String, String)> {
        let mut root = "";
        let mut result = vec![];
        for (i, s) in path.split(' ').enumerate() {
            if i == 0 {
                root = s;
            } else {
                let left = s.find("(").unwrap();
                result.push((
                    format!("{}/{}", root, &s[..left]),
                    s[left + 1..s.len() - 1].to_string(),
                ))
            }
        }
        result
    }

    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for path in paths.iter() {
            for (p, content) in Self::parse_path(path) {
                match map.get_mut(&content) {
                    None => {
                        map.insert(content, vec![p]);
                    }
                    Some(v) => v.push(p),
                }
            }
        }
        map.iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(_, v)| v.clone())
            .collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort(array: &mut Vec<Vec<String>>) {
        for r in array.iter_mut() {
            r.sort();
        }
        array.sort();
    }

    fn correct(paths: &Vec<&str>, expected: &Vec<Vec<&str>>) {
        let mut result = Solution::find_duplicate(paths.iter().map(|x| x.to_string()).collect())
            .iter()
            .map(|r| r.iter().map(|x| x.to_string()).collect())
            .collect();
        let mut expected = expected
            .iter()
            .map(|r| r.iter().map(|x| x.to_string()).collect())
            .collect();
        sort(&mut expected);
        sort(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_1() {
        let paths = vec![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)",
        ];
        let expected = vec![
            vec!["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            vec!["root/a/1.txt", "root/c/3.txt"],
        ];
        correct(&paths, &expected);
    }

    #[test]
    fn example_2() {
        let paths = vec![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
        ];
        let expected = vec![
            vec!["root/a/2.txt", "root/c/d/4.txt"],
            vec!["root/a/1.txt", "root/c/3.txt"],
        ];
        correct(&paths, &expected);
    }
}
