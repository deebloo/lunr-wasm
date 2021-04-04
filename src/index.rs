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
    pub documents: Vec<String>,
    pub inverted_index: HashMap<String, Vec<usize>>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            documents: vec![],
            inverted_index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, document: String) {
        // keep track of standard document
        self.documents.push(document.clone());

        let document_index = self.documents.len() - 1;

        self.index_document(document, document_index);
    }

    pub fn search(&self, query: String) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        if let Some(entry) = self.inverted_index.get(&query) {
            for i in entry {
                let document = self.documents[*i].clone();

                results.push(document)
            }
        }

        results
    }

    // split document into words and popular inverted index
    fn index_document(&mut self, document: String, document_index: usize) {
        for word in document.split_whitespace() {
            let mut normalized_word = word.to_lowercase();

            normalized_word = normalized_word.trim_matches(PUNCUATION).to_string();

            if !STOP_WORDS.contains(&normalized_word.as_str()) {
                self.inverted_index
                    .entry(normalized_word)
                    .or_insert(vec![])
                    .push(document_index);
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

        index.add_document("Hello World!?.".to_string());

        assert_eq!(index.documents, vec!["Hello World!?."]);

        let mut expected_inverted: HashMap<String, Vec<usize>> = HashMap::new();
        expected_inverted.insert("hello".to_string(), vec![0 as usize]);
        expected_inverted.insert("world".to_string(), vec![0 as usize]);

        assert_eq!(index.inverted_index, expected_inverted);
    }

    #[test]
    fn should_ignore_common_words() {
        let mut index = Index::new();

        // "the", "be", "to", "of", "and", "a", "in", "that", "have", "i"
        index.add_document(STOP_WORDS.join(" ").to_string() + " Hello World");

        assert_eq!(
            index.documents,
            vec!["the be to of and a in that have i Hello World"]
        );

        let mut expected_inverted: HashMap<String, Vec<usize>> = HashMap::new();
        expected_inverted.insert("hello".to_string(), vec![0 as usize]);
        expected_inverted.insert("world".to_string(), vec![0 as usize]);

        assert_eq!(index.inverted_index, expected_inverted);
    }

    #[test]
    fn should_perform_simple_search() {
        let mut index = Index::new();
        index.add_document("Hello World".to_string());
        index.add_document("Goodbye World".to_string());
        index.add_document("Foo Bar".to_string());

        let res = index.search("world".to_string());

        assert_eq!(res, vec!["Hello World", "Goodbye World"]);
    }
}
