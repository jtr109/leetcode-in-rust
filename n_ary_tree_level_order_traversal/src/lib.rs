pub fn level_order(root: Vec<Option<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut current_level_cache = vec![];
    let mut previous_level_node_count = 1;
    let mut current_level_none_count = 0;
    for n in root {
        match n {
            Some(v) => {
                current_level_cache.push(v);
                continue;
            }
            None => {
                current_level_none_count += 1;
                if current_level_none_count < previous_level_node_count {
                    continue;
                }
                // the end of current level
                res.push(current_level_cache.clone());
                previous_level_node_count = current_level_cache.len();
                current_level_cache = vec![];
                current_level_none_count = 0;
            }
        }
    }
    res.push(current_level_cache);
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let root = vec![
            Some(1),
            None,
            Some(3),
            Some(2),
            Some(4),
            None,
            Some(5),
            Some(6),
        ];
        let expected = vec![vec![1], vec![3, 2, 4], vec![5, 6]];
        assert_eq!(level_order(root), expected);
    }
}
