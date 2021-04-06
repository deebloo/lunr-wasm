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

    pub fn add_document(&mut self, document_id: &str, document: &str) {
        // Index document for search
        self.index_document(document_id.to_string(), document.to_string());
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let normalized_query = query.to_lowercase();
        let parsed_query = normalized_query.split_whitespace();

        let mut results: Vec<String> = vec![];

        for query_part in parsed_query {
            if let Some(entry) = self.inverted_index.get(&query_part.to_string()) {
                for i in entry {
                    results.push(i.clone());
                }
            }
        }

        results
    }

    pub fn load_index(&mut self, encoded_index: Vec<u8>) {
        let decoded: InvertedIndex = bincode::deserialize(&encoded_index).unwrap();

        self.inverted_index = decoded
    }

    pub fn export_index(&self) -> Vec<u8> {
        let encoded: Vec<u8> = bincode::serialize(&self.inverted_index).unwrap();

        encoded
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

        let res = index.search("World");

        assert_eq!(res, vec!["0", "1"]);
    }
}
