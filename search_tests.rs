use megastore_projeto2::{Product, SearchEngine};

fn build_engine() -> SearchEngine {
    let mut engine = SearchEngine::new();
    engine.add_product(Product::new(1, "Smartphone Galaxy A55", "Samsung", "Eletronicos", "Celular"));
    engine.add_product(Product::new(2, "Notebook Inspiron 15", "Dell", "Informatica", "Notebook"));
    engine.add_product(Product::new(3, "Fone Bluetooth Bass", "Sony", "Eletronicos", "Fone"));
    engine
}

#[test]
fn searches_product_by_name() {
    let engine = build_engine();
    let results = engine.search_by_name("notebook");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].brand, "Dell");
}

#[test]
fn searches_products_by_category() {
    let engine = build_engine();
    let results = engine.search_by_category("eletronicos");
    assert_eq!(results.len(), 2);
}

#[test]
fn searches_product_by_brand_case_insensitive() {
    let engine = build_engine();
    let results = engine.search_by_brand("SAMSUNG");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Smartphone Galaxy A55");
}

#[test]
fn returns_empty_when_product_is_not_found() {
    let engine = build_engine();
    let results = engine.search_all("geladeira");
    assert!(results.is_empty());
}
