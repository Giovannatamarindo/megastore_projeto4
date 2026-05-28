use megastore_projeto2::{Product, SearchEngine};

fn main() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product::new(
        1,
        "Smartphone Galaxy A55",
        "Samsung",
        "Eletronicos",
        "Celular com 128 GB de armazenamento",
    ));
    engine.add_product(Product::new(
        2,
        "Notebook Inspiron 15",
        "Dell",
        "Informatica",
        "Notebook para estudos e trabalho",
    ));
    engine.add_product(Product::new(
        3,
        "Fone Bluetooth Bass",
        "Sony",
        "Eletronicos",
        "Fone sem fio com bateria de longa duracao",
    ));

    println!("Busca por categoria: Eletronicos");
    for product in engine.search_by_category("eletronicos") {
        println!("{} - {} ({})", product.id, product.name, product.brand);
    }

    println!("\nBusca geral: Dell");
    for product in engine.search_all("dell") {
        println!("{} - {} ({})", product.id, product.name, product.brand);
    }
}
