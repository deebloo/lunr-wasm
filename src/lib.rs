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

impl Lunr {
    pub fn new() -> Lunr {
        Lunr {
            index: Index::new(),
        }
    }

    pub fn add_document(&mut self, document: String) {
        self.index.add_document(document);
    }

    pub fn search(&self, query: String) -> Vec<String> {
        self.index.search(query)
    }
}
