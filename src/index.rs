use crate::query::Query;

use std::collections::HashMap;

// list of puncuations
const PUNCUATION: &[char] = &[
    '.', '?', '!', ';', ':', ',', '(', ')', '[', ']', '{', '}', '"', '-',
];

// TOP 10 most common words
const STOP_WORDS: [&str; 10] = [
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
];

pub type InvertedIndex = HashMap<String, Vec<String>>;

pub struct Index {
    pub inverted_index: InvertedIndex,
}

impl Index {
    pub fn new() -> Index {
        Index {
            inverted_index: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn load_index(&mut self, encoded_index: Vec<u8>) {
        self.inverted_index = bincode::deserialize(&encoded_index).unwrap();
    }

    pub fn export_index(&self) -> Vec<u8> {
        bincode::serialize(&self.inverted_index).unwrap()
    }

    pub fn add_document(&mut self, document_id: &str, document: &str) {
        // Index document for search
        self.index_document(document_id.to_string(), document.to_string());
    }

    #[allow(dead_code)]
    pub fn search(&self, query: &str) -> Vec<String> {
        let parsed_query = Query::from_str(query);

        let mut results: HashMap<String, usize> = HashMap::new();
        let mut query_count = 0;

        for query_part in parsed_query.terms {
            query_count += 1;

            if let Some(entry) = self.inverted_index.get(&query_part) {
                for i in entry {
                    let res = results.entry(i.clone()).or_insert(0);

                    *res += 1;
                }
            }
        }

        results
            .into_iter()
            .filter_map(
                |(id, count)| {
                    if count == query_count {
                        Some(id)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    // split document into words and popular inverted index
    fn index_document(&mut self, document_id: String, document: String) {
        let parsed_document = document.split_whitespace();

        for word in parsed_document {
            let normalized_word = word.to_lowercase();
            let trimmed_word = normalized_word.trim_matches(PUNCUATION);

            if !STOP_WORDS.contains(&trimmed_word) {
                let entry = self
                    .inverted_index
                    .entry(trimmed_word.to_string())
                    .or_insert(vec![]);

                entry.push(document_id.clone());
                entry.dedup();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_index_document() {
        let mut index = Index::new();

        index.add_document("0", "Hello World!?.");

        let mut expected_inverted: HashMap<String, Vec<String>> = HashMap::new();
        expected_inverted.insert("hello".to_string(), vec!["0".to_string()]);
        expected_inverted.insert("world".to_string(), vec!["0".to_string()]);

        assert_eq!(index.inverted_index, expected_inverted);
    }

    #[test]
    fn should_ignore_common_words() {
        let mut index = Index::new();

        // "the", "be", "to", "of", "and", "a", "in", "that", "have", "i"
        index.add_document("0", STOP_WORDS.join(" ").as_str());

        let expected_inverted: HashMap<String, Vec<String>> = HashMap::new();

        assert_eq!(index.inverted_index, expected_inverted);
    }

    #[test]
    fn should_perform_simple_search() {
        let mut index = Index::new();
        index.add_document("0", "Hello World");
        index.add_document("1", "Goodbye World");
        index.add_document("2", "Foo Bar");

        let mut res = index.search("World");

        res.sort();

        assert_eq!(res, vec!["0", "1"]);
    }

    #[test]
    fn should_treat_as_and_query() {
        let mut index = Index::new();
        index.add_document("0", "Hello World Foo Bar");
        index.add_document("1", "Goodbye World");
        index.add_document("2", "Foo Bar");

        let mut res = index.search("Hello World");

        res.sort();

        assert_eq!(res, vec!["0"]);

        let mut res = index.search("Foo Bar");

        res.sort();

        assert_eq!(res, vec!["0", "2"]);
    }
}
