mod index;

use index::Index;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Lunr {
    index: Index,
}

#[wasm_bindgen]
impl Lunr {
    pub fn new() -> Lunr {
        Lunr {
            index: Index::new(),
        }
    }

    pub fn add_document(&mut self, document_id: &str, document: &str) {
        self.index.add_document(document_id, document);
    }

    pub fn search(&self, query: &str) -> String {
        self.index.search(query).join(",")
    }
}
