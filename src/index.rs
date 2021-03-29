type FieldVector = (String, Vec<f32>);

pub struct Index {
    pub fields: Vec<String>,
    pub field_vectors: Vec<FieldVector>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            fields: vec![],
            field_vectors: vec![],
        }
    }

    pub fn add_document(&mut self) {}
}
