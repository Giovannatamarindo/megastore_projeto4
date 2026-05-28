use std::collections::{HashMap, HashSet};

use crate::product::Product;

#[derive(Debug, Default)]
pub struct SearchEngine {
    products: HashMap<u32, Product>,
    name_index: HashMap<String, HashSet<u32>>,
    brand_index: HashMap<String, HashSet<u32>>,
    category_index: HashMap<String, HashSet<u32>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_product(&mut self, product: Product) {
        let id = product.id;
        self.index_text(&product.name, id, IndexType::Name);
        self.index_text(&product.brand, id, IndexType::Brand);
        self.index_text(&product.category, id, IndexType::Category);
        self.products.insert(id, product);
    }

    pub fn search_by_name(&self, term: &str) -> Vec<&Product> {
        self.search(&self.name_index, term)
    }

    pub fn search_by_brand(&self, term: &str) -> Vec<&Product> {
        self.search(&self.brand_index, term)
    }

    pub fn search_by_category(&self, term: &str) -> Vec<&Product> {
        self.search(&self.category_index, term)
    }

    pub fn search_all(&self, term: &str) -> Vec<&Product> {
        let normalized = normalize(term);
        let mut ids = HashSet::new();

        for index in [&self.name_index, &self.brand_index, &self.category_index] {
            if let Some(found_ids) = index.get(&normalized) {
                ids.extend(found_ids);
            }
        }

        let mut results: Vec<&Product> = ids
            .iter()
            .filter_map(|id| self.products.get(id))
            .collect();
        results.sort_by_key(|product| product.id);
        results
    }

    fn search<'a>(&'a self, index: &HashMap<String, HashSet<u32>>, term: &str) -> Vec<&'a Product> {
        let normalized = normalize(term);
        let mut results: Vec<&Product> = index
            .get(&normalized)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.products.get(id))
            .collect();
        results.sort_by_key(|product| product.id);
        results
    }

    fn index_text(&mut self, text: &str, id: u32, index_type: IndexType) {
        for token in tokenize(text) {
            let index = match index_type {
                IndexType::Name => &mut self.name_index,
                IndexType::Brand => &mut self.brand_index,
                IndexType::Category => &mut self.category_index,
            };
            index.entry(token).or_insert_with(HashSet::new).insert(id);
        }
    }
}

#[derive(Copy, Clone)]
enum IndexType {
    Name,
    Brand,
    Category,
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}

fn tokenize(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(normalize)
        .filter(|token| !token.is_empty())
        .collect()
}
