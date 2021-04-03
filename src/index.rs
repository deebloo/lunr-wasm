use std::collections::HashMap;

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

        // split document into words and popular inverted index
        for word in document.split_whitespace() {
            let mut normalized_word = word.to_lowercase();

            normalized_word = normalized_word
                .trim_matches('.')
                .trim_matches('?')
                .trim_matches('!')
                .to_string();

            self.inverted_index
                .entry(normalized_word)
                .or_insert(vec![])
                .push(document_index);
        }
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
    fn should_perform_simple_search() {
        let mut index = Index::new();
        index.add_document("Hello World".to_string());
        index.add_document("Goodbye World".to_string());
        index.add_document("Foo Bar".to_string());

        let res = index.search("world".to_string());

        assert_eq!(res, vec!["Hello World", "Goodbye World"]);
    }
}
