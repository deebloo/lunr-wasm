use std::collections::HashMap;

// list of puncuations
const PUNCUATION: &[char] = &[
    '.', '?', '!', ';', ':', ',', '(', ')', '[', ']', '{', '}', '"', '-',
];

// TOP 10 most common words
const STOP_WORDS: [&str; 10] = [
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
];

pub struct Index {
    pub documents: HashMap<String, String>,
    pub inverted_index: HashMap<String, Vec<String>>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            documents: HashMap::new(),
            inverted_index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, document_id: String, document: String) {
        // keep track of standard document
        self.documents.insert(document_id.clone(), document.clone());

        self.index_document(document_id, document);
    }

    // split document into words and popular inverted index
    fn index_document(&mut self, document_id: String, document: String) {
        for word in document.split_whitespace() {
            let normalized_word = word.to_lowercase();
            let trimmed_word = normalized_word.trim_matches(PUNCUATION);

            if !STOP_WORDS.contains(&trimmed_word) {
                self.inverted_index
                    .entry(trimmed_word.to_string())
                    .or_insert(vec![])
                    .push(document_id.clone());
            }
        }
    }

    pub fn search(&self, query: String) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        if let Some(entry) = self.inverted_index.get(&query.to_lowercase()) {
            for i in entry {
                let document = self.documents.get(i).unwrap();

                results.push(document.clone())
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_index_document() {
        let mut index = Index::new();

        index.add_document("0".to_string(), "Hello World!?.".to_string());

        let mut expected_documents: HashMap<String, String> = HashMap::new();
        expected_documents.insert("0".to_string(), "Hello World!?.".to_string());

        assert_eq!(index.documents, expected_documents);

        let mut expected_inverted: HashMap<String, Vec<String>> = HashMap::new();
        expected_inverted.insert("hello".to_string(), vec!["0".to_string()]);
        expected_inverted.insert("world".to_string(), vec!["0".to_string()]);

        assert_eq!(index.inverted_index, expected_inverted);
    }

    #[test]
    fn should_ignore_common_words() {
        let mut index = Index::new();

        // "the", "be", "to", "of", "and", "a", "in", "that", "have", "i"
        index.add_document(
            "0".to_string(),
            STOP_WORDS.join(" ").to_string() + " Hello World",
        );

        let mut expected_documents: HashMap<String, String> = HashMap::new();
        expected_documents.insert(
            "0".to_string(),
            "the be to of and a in that have i Hello World".to_string(),
        );

        assert_eq!(index.documents, expected_documents);

        let mut expected_inverted: HashMap<String, Vec<String>> = HashMap::new();
        expected_inverted.insert("hello".to_string(), vec!["0".to_string()]);
        expected_inverted.insert("world".to_string(), vec!["0".to_string()]);

        assert_eq!(index.inverted_index, expected_inverted);
    }

    #[test]
    fn should_perform_simple_search() {
        let mut index = Index::new();
        index.add_document("0".to_string(), "Hello World".to_string());
        index.add_document("1".to_string(), "Goodbye World".to_string());
        index.add_document("2".to_string(), "Foo Bar".to_string());

        let res = index.search("World".to_string());

        assert_eq!(res, vec!["Hello World", "Goodbye World"]);
    }
}
