pub struct Query {
    pub terms: Vec<String>,
}

impl Query {
    #[allow(dead_code)]
    pub fn new() -> Query {
        Query { terms: vec![] }
    }

    pub fn from_str(query: &str) -> Query {
        let normalized_query = query.to_lowercase();
        let parsed_query = normalized_query.split_whitespace();

        Query {
            terms: parsed_query.map(|t| t.to_string()).collect(),
        }
    }
}
