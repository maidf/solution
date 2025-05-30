pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();

    strs.iter().for_each(|word| {
        let mut chars = word.chars().collect::<Vec<char>>();
        chars.sort();
        let key = chars.iter().collect::<String>();
        map.entry(key).or_insert(Vec::new()).push(word.clone());
    });

    map.into_iter().map(|(_, v)| v ).collect()
}

#[cfg(test)]
mod test {
    use super::group_anagrams;

    #[test]
    fn test() {
        let strs =
            vec!["eat", "tea", "tan", "ate", "nat", "bat"].iter().map(|&s| s.to_string()).collect();
        let res = group_anagrams(strs);
        println!("res: {res:?}")
    }
}
