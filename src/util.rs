// list of puncuations
const PUNCUATION: &[char] = &[
    '.', '?', '!', ';', ':', ',', '(', ')', '[', ']', '{', '}', '"', '-',
];

pub fn trimmer(word: String) -> String {
    word.trim_matches(PUNCUATION).to_string()
}

pub fn find_intersection(a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    let mut larger_list = a;
    let mut smaller_list = b;

    if a.len() < b.len() {
        larger_list = b;
        smaller_list = a;
    }

    for id in larger_list {
        if smaller_list.contains(&id) {
            results.push(id.clone());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_differences() {
        let res = find_intersection(
            &vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ],
            &vec![
                "0".to_string(),
                "B".to_string(),
                "C".to_string(),
                "3".to_string(),
            ],
        );

        assert_eq!(res, vec!["0".to_string(), "3".to_string(),]);
    }
}
